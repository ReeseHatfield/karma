#!/bin/bash

clear

TOPIC="$1"


mkdir -p output
rm tmp_py_input.txt

echo "TOPIC was $TOPIC"

cargo run -- $TOPIC | tee "output/$TOPIC.txt"

# the rust ollama bindings do not free the process, must be killed manually
sudo kill $(pgrep ollama)

cat "output/$TOPIC.txt" | grep VIS > tmp_py_input.txt

cd visualization
source venv/bin/activate
pip install networkx matplotlib
python3 src/vis.py ../tmp_py_input.txt
