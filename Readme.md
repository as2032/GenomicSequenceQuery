# Genomic Sequnce Query

## Introduction
This program builds suffix arrays and quieries them using naive and accelerated binary search

### Suffix Array Construction

To construct the suffix array, I leveraged Rust's SuffixTable. This library provides efficient tools for working with suffixes, which were essential for this project.
* SuffixTable: https://docs.rs/suffix/latest/suffix/index.html

### Executables

This program consists of two main executables, buildsa and querysa. 

* buildsa: The code for buildsa can be found in src/main.rs.
* querysa: The code for querysa resides in src/bin/main2.rs.

To build these executables, use the following commands:
'''
cargo build --bin buildsa --release
cargo build --bin querysa --release
'''
