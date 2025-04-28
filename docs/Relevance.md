# Relevance

There are two primary areas of significance that KARMA [BRINGS ABOUT? fix wording]

1. Knowledge Verification
2. Internal Knowledge Crawling

## Knowledge Verification
Triples are verified against their existence in the retrieved set of objecst from WikiData. The retreived set can be reproduced via the following SPARQL and code snippet:
```rust
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
```
If any of the generated objects are within the set of entities retrieved from the SPARQL query, they become added to the pool of potential new topics to crawl. This treats WikiData as a ground truth for knowledge. This idea of verifying generated knowledge with an external knowledge graph can be more broadly applied to help curb LLM hallucinations.

## Internal Knowledge Crawling
KARMA traverses knowledge by using a fresh context window of a language model. Every time a new (verifiably true) topic is chosen, a fresh context window is created, and more tuples are generated. This process continues until no verifiably true facts are left to explore. This is relevant because it gives some insight into the explainabilty of language models. Crawling data in this fashion increases the transparency behind how the model conceptualizes facts.
