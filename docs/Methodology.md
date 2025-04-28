# Prompt Methodology

The prompting methodogy for triple extraction from a LLM for this project can be broken down into several parts:

## Task Exposition
Explaining what the task is, in this case, what it should actually be extracting, helps the model understand the bigger picture in regards to its goals. "I need you to extract information about {} and represent it as a set of knowledge graph triples" gives the model a good baseline of understand and context for the remainder of the prompt.

## Definitions
The most notable thing that needed to be defined for this usecase was the idea of a knowledge graph triple. Although some models had a solid baseline understanding of this, some smaller models tested benefit, strictly defining what a triple is and all its pieces with "[Subject] -> [Predicate] -> [Object] ... A triple is a structured fact where the subject is connected to the object by a relationship (predicate). Each triple should represent a single, clear piece of knowledge."

## Assurance of Relevancy
An additional bit of reenforcement helps ensure that the model output stays on track. The section of the prompt consisting of "Focus on factual, non-redundant triples" was especially important in reducing examples of knowledge that were less relevant or essentially duplicates.

## Example
The most important helpful addition to the prompt was the inclusion of an example. The example used is regarding "Photosynthesis." This was an arbitrary decision, but the more important part of this is it gives the model some understanding of what it's output should strictly look like. However, some models still have trouble getting the output format to match the given formatting.

## Generation Request
The final piece of the prompt involves an additional generation request: "Now, please give me the triples for the topic." Without this additional request, some smaller models would refuse to generate triples at all, opting instead to just give plaintext information regarding the topic

## Final Prompt:

The final prompt that was settled on was:
```
I need you to extract information about {} and represent it as a set of knowledge graph triples in the following format:
[Subject] -> [Predicate] -> [Object]

A triple is a structured fact where the subject is connected to the object by a relationship (predicate). Each triple should represent a single, clear piece of knowledge.

Make sure the triples are relevant and specific to the topic, with clearly identified entities and meaningful relationships. Focus on factual, non-redundant triples that would be useful for building a knowledge graph.

For example, for the topic 'Photosynthesis':
Photosynthesis -> occurs in -> Chloroplasts
Photosynthesis -> uses -> Sunlight
Photosynthesis -> produces -> Oxygen
Plants -> perform -> Photosynthesis

Now, please give me the triples for the topic: {}
```
