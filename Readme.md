This solution for Programming Assignment 1 was written in Rust as it provides decent efficiency while providing access to 3rd party library via the Rust CRATE. 

To build the suffix array, I utilized Rust's SuffixTable from
https://docs.rs/suffix/latest/suffix/index.html

Other than this, to build the rest of the project I utilized class slides/lectures and the basic Rust documentation for various functions/built-in libraries at https://docs.rs/

This solution for Programming Assignment 1 was written in Rust as it provides decent efficiency while providing access to 3rd party library via the Rust CRATE. 

To build the suffix array, I utilized Rust's SuffixTable from
https://docs.rs/suffix/latest/suffix/index.html

Other than this, to build the rest of the project I utilized class slides/lectures and the basic Rust documentation for various functions/built-in libraries at https://docs.rs/

This program consists of two main executables, buildsa and querysa. 
The code for buildsa is found in src/main.rs
The code for querysa is found in src/bin/main2.rs


executables were made using cargo build --bin buildsa --release and cargo build --bin querysa --release, and then moving them to the top level of the directory. This choice was made to preserve directory order.