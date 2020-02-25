#!/bin/bash

for file in $(ls ./src/tests/*.mmd); do
  docker run -v "$(pwd)/src/tests:/tests" jnewland/mermaid.cli -i /tests/$(basename $file)
done 

