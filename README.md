
<p align="center">
  <a href="https://github.com/reesehatfield/karma">
    <img src="./docs/img/Logo.png" alt="Karma Logo" width="400" height="370">
  </a>
</p>

<h3 align="center"><strong>KARMA</strong></h3>

<p align="center">
    Knowledge Association and Reasoning Mapping Agent
  <br>
</p>

## What is KARMA?
KARMA is a large language model (LLM) internal knowledge-based fact crawler. KARMA is a dynamic system that can crawl semantically related concepts in an LLM. Facts are checked verified against existing an knowledge base, using the WikiData API. KARMA will walk through facts about a given topic, dynamically selecting new facts based on the LLMs perceived relationship to one another.

## Examples
<p align="center">
  <a href="https://github.com/reesehatfield/karma">
    <img src="./docs/img/wsu.png" alt="WSU KG" width="550" height="370">
  </a>
</p>
<p align="center">
    Query for Wright State University
  <br>
</p>
<!-------------------->
<p align="center">
  <a href="https://github.com/reesehatfield/karma">
    <img src="./docs/img/einstein.png" alt="WSU KG" width="550" height="370">
  </a>
</p>
<p align="center">
    Query for Albert Einstein
  <br>
</p>


## Usage
To use KARMA, just run the shell script with the topic as the first argument (do not use spaces)

```bash
# crawl LLM knowledge related to Albert Einstein
./run.sh "AlbertEinstein"
# see output image in ./visualization/graph_output.png
# bat visualization/graph_output.png
```

## Prerequisites


| Requirement | Purpose                          |
|:------------|:---------------------------------|
| Rust        | Implementation language          |
| Cargo       | Dependency and build manager     |
| Python      | Visualization language           |
| Pip         | Visualization dependency manager |
| Ollama      | LLM backend                      |

```bash
# Install mistral with Ollama
ollama pull mistral
```


## Docs

[Knowledge Crawling](./docs/Crawling.md)

[Future Work](./docs/FutureWork.md)

[Prompting Methodology](./docs/PromptingMethodology.md)

[Relevance](./docs/Relevance.md)

[Models](./docs/SupportedModels.md)
