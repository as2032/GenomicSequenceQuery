# Genomic Sequence Query

## Introduction
This program builds suffix arrays and quieries them using naive and accelerated binary search

### Suffix Array Construction

To construct the suffix array, I leveraged Rust's SuffixTable. This library provides efficient tools for working with suffixes, which were essential for this project.
* SuffixTable: https://docs.rs/suffix/latest/suffix/index.html

### Building Executables

This program consists of two main executables, buildsa and querysa. 

* buildsa: The code for buildsa can be found in src/main.rs.
* querysa: The code for querysa resides in src/bin/main2.rs.

To build these executables, use the following commands:
```
cargo build --bin buildsa --release
cargo build --bin querysa --release
```

### Example Use

#### Build SA
##### Arguments and Example Use
1. --preftab <k> if the option --preftab is passed to the buildsa executable (with the parameter k), then a prefix table will be built atop the suffix array, capable of jumping to the suffix array interval corresponding to any prefix of length k
2. reference  the path to a FASTA format file containing the reference of which the suffix array will be built
3. output - the program will write a single binary output file to a file with this name, that contains a serialized version of the input string and the suffix array.
```
./buildsa --preftab 5 input.fasta output.bin
```
#### Query SA
#### Arguments and Example Use
1. index - the path to the binary file containing your serialized suffix array (as written by buildsa above)
2. queries - the path to an input file in FASTA format containing a set of records.
   An example of a queries file can be found in the queries folder
4. query mode - this argument should be one of two strings; either naive or simpaccel. If the string is naive queries will be performed using the naive binary search algorithm. If the string is simpaccel queries will be performed using an accelerated binary search algorithm
5. output - the name to use for the resulting output
```
./querysa output.bin queries.fasta naive searchoutput.txt
'''
#### Output
The output text file contains the results of the query in the following format
```
query_name, k, hit_1, hit_2, hit_k
```


