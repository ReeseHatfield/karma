# Supported Large Language Models
A variety of local large language models were tested in with KARMA.
Due to hardware limitations, the number of params was limited. The upper bound used capped out around the 8 billion mark with one the smaller publically available deepseek-r1 models. Overall, the prototype used for exploration in this model was Mistral


| Code Enumeration    | Ollama Model Name                      | Parameters |
|--------------------|-----------------------------------------|------------|
| Deepseek           | deepseek-r1                             | ~8B        |
| LlammaThree        | hf.co/prithivMLmods/Llama-3.2-1B-GGUF   | ~1B        |
| Triplex            | sciphi/triplex                          |~3B         |
| GPT3               | mapler/gpt2                             | ~1.5B      |
| Mistral            | mistral                                 | 7B         |

## Why was Mistral Chosen for KARMA?

Mistral was chosen for its overall more consistent. Frequently, other models showed signs of significant misalignment with the given prompt. The prompt was tweaked slightly in all cases, but we settled on using the [provided prompt](./PromptingMethodology.md). The decision to go with Mistral can ultimately be better summarized by why the other models were *not* Chosen

### Deepseek
Deepseek performed well on initial tests, but the due to the nature of its output, it became needlessly obtuse to parse. Deepseek claims to be a "Reasoning Model". Deepseek will output a large "<think> [THOUGHTS] </think> block before the actual model output. Although this can be useful, it often spends an large amount of processing power on its <think></think> and chews through time at an unreasonable rate for testing.

### LlammaThree
LlamaThree showed significant signs of misalignment. Despite a clear and descriptive prompt, LlamaThree would often "not listen" to the task of triple generation, and instead reply with plaintext, unstructured facts.

### GPT2
GPT2 also showed significant signs of misaligment, displaying surprisingly similar results to LlammaThree. This is liekly due to the lower number of parameters that these models contained

### Triplex
Triplex is different compared to the rest of the models tested. Triplex specializes in generating triples from unstructured text. Originally, the implementation plan involved having a LLM generate unstructed knowledge about a topic, then that output would be fed through Triplex to turn the text into structured data. However, this proved to be ineffective due to Triplex requiring a schema at time of system prompt. This schema can not be effectively generated for all possible knowledge, so it was not used

| **Model**      | **Rejection Criteria**              |
|----------------|-------------------------------------|
| **Deepseek**   | Verbosity, slow output              |
| **LlamaThree** | Misaligned, unstructured responses  |
| **GPT-2**      | Misaligned, unstructured responses  |
| **Triplex**    | Requires schema, inflexible         |
