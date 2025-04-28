#!/bin/bash

SYS_PROMPT="You will be responsible for generating a set of knowledge graph triples from the text. Generate each triple as (subject, predicate, object). Ensure clarity and accuracy in the extraction. Please ensure that you actually do extract reala triples and do not reply with an empty output"

echo -e "$SYS_PROMPT\n\n$(cat Germany.txt)" | ollama run sciphi/triplex
