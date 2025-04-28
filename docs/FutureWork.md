# Future Work

## Configurable Models
While KARMA has support for multiple different models, (see code snippet), at present moment, this is not configurable to the end user. In theory, this could be expanded to support any Ollama supported model. This could feasibly be done at runtime, downloading the model if required. Presently, this requires the model already be downloaded.
```rust
pub enum SupportedModels {
    Deepseek,
    LlammaThree,
    Triplex,
    GPT3,
    Mistral,
}

impl SupportedModels {
    pub fn to_string(&self) -> String {
        match self {
            SupportedModels::Deepseek => "deepseek-r1".to_string(),
            SupportedModels::LlammaThree => "hf.co/prithivMLmods/Llama-3.2-1B-GGUF".to_string(),
            SupportedModels::Triplex => "sciphi/triplex".to_string(),
            SupportedModels::GPT3 => "mapler/gpt2".to_string(),
            SupportedModels::Mistral => "mistral".to_string(),
        }
    }
}

```

## Predicate Verification
Presently, when the model generates a series of predicted triples, only the objects of the triples are verified against the WikiData KG. This leads to a string possibilty of the traversal graph being disconnected, despite the entities being semantically related.

## WikiData Global Verification vs Retrieved Verification
Presently, there exists code to swap between verifying objects through a SPARQL query for the topic or general existence in WikiData. We chose to use the verification that checks against only objects retrieved by the SPARQL query, as this led to more connected output traversal graphs. This option could trivially be made configurable to the end user.


## Usability
Presently, the way the user provides a topic is unintuitive. There is a bug in `run.sh` that prevents spaces from being passed into the argument.

## Ranked Predictions
Presently, the choice of which predicate to follow for the next topic is arbitrary. KARMA choses a random outlink to simulate a random walk of the models internal, verifiable knowledge. There is value in this, but often quickly results in termination. I believe it would be possible to make this decision in a more purposeful manner. If you could construct a system for ranking predicates by *actual* relevance between the curent topic and the object, you could pick the next topic and explore triples more relevant to the starting topic.

## Improved Object Verification
Presently, the system in place for verifcation of objects in WikiData is a fuzzy search using their API.
```rust
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
```
This system works very well for preventing generated objects that are not in WikiData from ending up in the pool for potential new topics. However, this search is fuzzy and is often inaccurate. For example, when searching for the entity "Micheal Jackson", the function returns "Maithai Huajaisin". This is likely due to the description of "Maithai Huajaisin" containing the phrase "Michael Jackson of Isan". This is indicative of a larger issues with the way entities are verified.
