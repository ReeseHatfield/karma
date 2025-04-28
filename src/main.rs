use regex::Regex;

use rand::{
    seq::{IndexedRandom, SliceRandom},
    thread_rng,
};
use reqwest::Client;
use serde::Deserialize;
use std::{env, error::Error, fs, sync::Arc};

use llm::{LLMProducer, NO_SCHEMA};
use types::TopicPath;
use wikidata::WikiData;

mod llm;
mod types;
mod wikidata;
#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let topic = args.get(1).unwrap();

    let wikidata = WikiData::new();
    let mut cur_topic: String = topic.to_string();

    println!("Starting topic was {}", topic);

    let use_global_wikidata_search = true;

    // big crawling loop
    loop {
        let topic_id = match get_wikidata_id(&cur_topic).await {
            Ok(Some(id)) => id,
            Ok(None) => {
                eprintln!("Couldn't find Wikidata ID for topic: {}", cur_topic);
                break;
            }
            Err(e) => {
                eprintln!("Error retrieving Wikidata ID for '{}': {}", cur_topic, e);
                break;
            }
        };

        println!("\n Topic ID: {}\n", topic_id);

        let query = wikidata::KgSPARQLQuery::new(format!(
            r#"
            SELECT ?subject ?predicate ?object WHERE {{
                ?subject ?predicate ?object.
                FILTER (?subject = wd:{})
                ?subject rdfs:label ?subjectLabel.
                FILTER(LANG(?subjectLabel) = "en")
                ?object rdfs:label ?objectLabel.
                FILTER(LANG(?objectLabel) = "en")
            }}
        "#,
            topic_id
        ));

        let wikidata_objects = match wikidata.submit_query(&query).await {
            Ok(triples) => {
                println!("Retrieved Wikidata Triples:\n");
                triples
                    .into_iter()
                    .map(|t| {
                        println!(
                            "- {} → {} → {}",
                            t.subject.value, t.predicate.value, t.object.value
                        );
                        t.object.value
                    })
                    .collect()
            }
            Err(e) => {
                eprintln!("Error querying Wikidata: {}", e);
                vec![] // bad
            }
        };

        let prompt = generate_prompt(&cur_topic);
        let model = llm::LLMProducer::new(
            llm::SupportedModels::Mistral,
            llm::LLMConfig {
                socket: "http://127.0.0.1:11434".to_string(),
                output_schema: NO_SCHEMA.to_string(),
            },
        )
        .await
        .expect("LLM Init Failed");

        let output = model.prompt_model(&prompt).await.unwrap();
        let generated_triples = parse_triples_from_output(&output);

        let next_topic_result = process_triples(
            &generated_triples,
            &wikidata_objects,
            use_global_wikidata_search,
        )
        .await;

        if let Some((next_topic_id, path)) = next_topic_result {
            println!(
                "\nSwitching to new topic: {} ({})\n",
                next_topic_id, path.to_object_label
            );
            print!("Reasoning Path:");
            println!(
                "{} → {} → {}",
                path.from_subject, path.via_predicate, path.to_object_label
            );
            cur_topic = path.to_object_label.clone();
        } else {
            println!("No new related topics found. Done.");
            break;
        }
    }
}

fn generate_prompt(topic: &str) -> String {
    format!(
        r#"
You are an expert knowledge graph builder.

Given the topic "{}",
generate a list of 5–10 factual knowledge graph triples in the format:
[Subject] -> [Predicate] -> [Object]

Each triple should be concise, factual, and expressed in simple, unambiguous language.

Example:
Photosynthesis -> occurs in -> Chloroplasts
Germany -> located in -> Europe

Now, generate triples for: {}
"#,
        topic, topic
    )
}

#[derive(Debug)]
struct GenTriple {
    subject: String,
    predicate: String,
    object: String,
}

fn parse_triples_from_output(output: &str) -> Vec<GenTriple> {
    let full_date = Regex::new(r"^[A-Z][a-z]+ \d{1,2}, \d{4}$").unwrap();
    let year_only = Regex::new(r"^\d{4}$").unwrap();

    output
        .lines()
        .filter(|line| line.contains("->"))
        .filter_map(|line| {
            let cleaned_line = line.trim_start().splitn(2, ". ").nth(1).unwrap_or(line);
            let parts: Vec<&str> = cleaned_line.split("->").map(|s| s.trim()).collect();
            if parts.len() == 3 {
                let object = parts[2];
                if full_date.is_match(object) || year_only.is_match(object) {
                    println!("Skipping date-like object: {}", object);
                    return None;
                }
                Some(GenTriple {
                    subject: parts[0].to_string(),
                    predicate: parts[1].to_string(),
                    object: object.to_string(),
                })
            } else {
                None
            }
        })
        .collect()
}

async fn process_triples(
    triples: &[GenTriple],
    wikidata_objects: &[String],
    use_global_search: bool,
) -> Option<(String, TopicPath)> {
    let mut found_subjects = 0;
    let mut found_predicates = 0;
    let mut found_objects = 0;
    let mut potential_new_topics: Vec<(String, TopicPath)> = vec![];

    println!("\nSearching Entities for Generated Triples:\n");

    for triple in triples {
        let subject = search_entity(&triple.subject).await;
        let predicate = search_entity(&triple.predicate).await;

        println!("Searching for object: {}", triple.object);
        let object = search_entity(&triple.object).await;
        println!("Found search result {:?}", object);

        if subject.is_some() {
            found_subjects += 1;
        }
        if predicate.is_some() {
            found_predicates += 1;
        }

        if let Some(obj) = &object {
            found_objects += 1;

            if use_global_search {
                if search_entity(&triple.object).await.is_some() {
                    println!("adding new object to potential pool {}", obj);

                    let new_topic = TopicPath {
                        from_subject: triple.subject.clone(),
                        via_predicate: triple.predicate.clone(),
                        to_object_label: triple.object.clone(), // raw lable
                        to_object_id: obj.clone(),
                    };
                    println!("VIS: Potential new topic: {:?}", new_topic);

                    potential_new_topics.push((
                        obj.clone(), // Q number,
                        new_topic,
                    ));
                }
            } else {
                let expected_uri = format!("http://www.wikidata.org/entity/{}", obj);
                let is_in_wikidata_retreived = wikidata_objects.iter().any(|s| s == &expected_uri);

                if is_in_wikidata_retreived {
                    potential_new_topics.push((
                        obj.clone(), // Q[whatever numbers]
                        TopicPath {
                            from_subject: triple.subject.clone(),
                            via_predicate: triple.predicate.clone(),
                            to_object_label: triple.object.clone(), // raw label
                            to_object_id: obj.clone(),
                        },
                    ));
                }
            }
        }

        println!(
            "- {} → {} → {}",
            triple.subject, triple.predicate, triple.object
        );
    }

    print_statistics(
        triples.len(),
        found_subjects,
        found_predicates,
        found_objects,
    );

    let chosen_topic = potential_new_topics.choose(&mut thread_rng()).cloned();
    println!("VIS chosen topic: {:?}", chosen_topic);

    return chosen_topic;
}

fn print_statistics(total: usize, found_subjects: i32, found_predicates: i32, found_objects: i32) {
    println!("\nTriple Analysis Summary");
    println!("Total triples: {}", total);
    println!(
        "Found Subjects:   {} ({:.1}%)",
        found_subjects,
        (found_subjects as f64 / total as f64) * 100.0
    );
    println!(
        "Found Predicates: {} ({:.1}%)",
        found_predicates,
        (found_predicates as f64 / total as f64) * 100.0
    );
    println!(
        "Found Objects:    {} ({:.1}%)",
        found_objects,
        (found_objects as f64 / total as f64) * 100.0
    );
}

async fn search_entity(name: &str) -> Option<String> {
    let url = format!(
        "https://www.wikidata.org/w/api.php?action=wbsearchentities&search={}&language=en&format=json",
        name
    );

    let res = reqwest::get(&url)
        .await
        .ok()?
        .json::<serde_json::Value>()
        .await
        .ok()?;

    res["search"]
        .get(0)
        .and_then(|item| item["id"].as_str())
        .map(|s| s.to_string())
}

async fn get_wikidata_id(query: &str) -> Result<Option<String>, reqwest::Error> {
    #[derive(Deserialize)]
    struct SearchResponse {
        query: Query,
    }

    #[derive(Deserialize)]
    struct Query {
        search: Vec<SearchResult>,
    }

    #[derive(Deserialize)]
    struct SearchResult {
        title: String,
    }

    let url = format!(
        "https://www.wikidata.org/w/api.php?action=query&list=search&srsearch={}&format=json",
        query
    );

    let response = reqwest::get(&url).await?.json::<SearchResponse>().await?;
    Ok(response
        .query
        .search
        .into_iter()
        .find(|r| r.title.starts_with("Q"))
        .map(|r| r.title))
}
