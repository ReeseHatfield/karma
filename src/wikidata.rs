use std::error::Error;

use crate::types::{self, WikiDataTriple};
use reqwest::{Client, Response};
use serde::Deserialize;
use serde_json::from_str;

pub struct WikiData {
    pub endpoint_url: String,
}

#[derive(Debug, Deserialize)]
struct WikidataResult {
    results: Results,
}

#[derive(Debug, Deserialize)]
struct Results {
    bindings: Vec<types::WikiDataTriple>,
}

impl WikiData {
    pub async fn submit_query(
        &self,
        q: &KgSPARQLQuery,
    ) -> Result<Vec<WikiDataTriple>, Box<dyn Error>> {
        let client = Client::new();

        let params = [("query", q.raw_text.clone()), ("format", "json".to_owned())];

        let res = client
            .get(self.endpoint_url.clone())
            .query(&params)
            .header("User-Agent", "RustClient/1.0") // should lie here
            .send()
            .await?;

        if !res.status().is_success() {
            return Err("Could not get submit query".into());
        }

        let xml_body = res.text().await?;

        let json: WikidataResult = from_str(&xml_body)?;

        Ok(json.results.bindings)
    }
    pub fn new() -> Self {
        WikiData {
            endpoint_url: "https://query.wikidata.org/sparql".to_owned(),
        }
    }
}

pub struct KgSPARQLQuery {
    raw_text: String, // will expand this later
}

impl KgSPARQLQuery {
    // wanna pass in like topic and build the q from that me thinks
    pub fn new(text: String) -> Self {
        KgSPARQLQuery { raw_text: text }
    }
}
