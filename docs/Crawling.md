# Knowledge Crawling

## How does KARMA crawl knowledge?
KARMA uses an existing language model generate *perceived* knowledge graph triples about an existing topics.  (See [Prompting Methodology](./PromptingMethodology.md) for how this works).

For the purposes of this prototype, [Mistral](https://ollama.com/library/mistral) was used as the primary language model for triple generation, as it gave the more consistent results, when tested against other local language models.

Notable, when prompted to generate triples regarding a particular topic, the majority (not necessarily 100%) of the triples will take the form:
```
[TOPIC] -> generated_predicate -> generated_object
```
Because of the nature of how language models generate triples, this leads to a star-like knowledge pattern around the topic. Since models like Mistral are famously non-deterministic, it is unclear which triples will exactly be generated, or if the generated triples will even be factually accurate to the real world. It is because of this potential inaccurarcy, that the all of the generated objects are externally verified.

KARMA uses the WikiData knowledge base to verify the precense of the language models generated objects. This filters out any potential model hallucinations, with the consequence of potentially trimming out more obscure facts the the LLM knows about the chosen topic.

After this filtering has occured, KARMA will decide on a new object to make the topic of the the search. This process repeats itself until the language model exclusively generates triples that cannot be externally verified.
