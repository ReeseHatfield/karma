use std::error::Error;

use ollama_rs::{Ollama, generation::completion::request::GenerationRequest};

use lazy_static::lazy_static;

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

pub struct LLMConfig {
    pub socket: String,
    pub output_schema: String,
}

lazy_static! {
    pub static ref NO_SCHEMA: String = "".to_string();
    pub static ref DEFAULT_CONFIG: LLMConfig = LLMConfig {
        socket: "http://127.0.0.1:11434".to_string(),
        output_schema: NO_SCHEMA.to_string()
    };
}

#[derive(Debug)]
pub(crate) struct LLMProducer {
    producer: Ollama,
    pub model: String,
}

impl LLMProducer {
    pub async fn new(
        model: SupportedModels,
        conf: LLMConfig,
    ) -> Result<LLMProducer, Box<dyn Error>> {
        let ollama = Ollama::try_new(conf.socket)?;

        ollama
            .generate(GenerationRequest::new(model.to_string(), ""))
            .await?;

        let mut schema_prompt = "".to_string();

        if conf.output_schema != "" {
            schema_prompt = ("Please always give your output in the following".to_string()
                + &conf.output_schema)
        }

        println!("Finished creating {:?}", model.to_string());
        return Ok(LLMProducer {
            producer: ollama,
            model: model.to_string(),
        });
    }

    pub async fn prompt_model(&self, prompt: &str) -> Result<String, Box<dyn Error>> {
        let req = GenerationRequest::new(self.model.to_string(), prompt);
        let res = self.producer.generate(req).await?;

        return Ok(res.response);
    }
}
