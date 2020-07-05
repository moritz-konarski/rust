#!/bin/bash

pandoc *.md -o rust_notes.pdf -V papersize=a4 -V geometry:margin=1in --toc --toc-depth=3 -V fontsize=12pt
