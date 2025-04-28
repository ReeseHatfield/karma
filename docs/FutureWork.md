# Future Work

## Configurable Models
While KARMA has support for multiple different models, see code snippet
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

## WikiData Global Verification vs Retrieved Verification

## Usability


## Ranked Predictions
chose the best outlink

## Improved Object Verification
not just a fuzzy search
