#!bin/bash
#
IFS=$'\n'

for file in *.html.astro
do
  mv "$file" "${file%.html.astro}.astro"
done
