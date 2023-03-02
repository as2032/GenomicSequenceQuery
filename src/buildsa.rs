
use std::fs::File;
use std::env;
use suffix::SuffixTable;
use std::io::{BufRead, BufReader, BufWriter};
use std::collections::HashMap;
use bincode;
use std::time::{Instant};
use itertools::{Itertools, MultiProduct};
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
struct Data {
    full_sequence: String,
    suffix: Vec<usize>,
    size_of_suffix: usize,
    prefix_tab: Option<HashMap<String, Option<(i32,i32)>>>,
    prefix_len: Option<usize>
}
fn main() {

    let args: Vec<String> = env::args().collect();
    // let filename = &args[1];
    // let binary = &args[2];
    let mut prefix_k: Option<usize> = None;
    // if args.len() >3 && &args[3] == "--preftab"{
    //     prefix_k = Some(args[4].parse::<usize>().unwrap());
    //     bin_p = true;
    // }
    let mut filename = "";
    let mut binary = "";
    let mut bin_p = false;
    if args.len() >3 && &args[1] == "--preftab"{
        prefix_k = Some(args[2].parse::<usize>().unwrap());
        bin_p = true;
        filename = &args[3];
        binary = &args[4];
    }else{
        filename = &args[1];
        binary = &args[2];
    }
    let mut full_file_path = "input_files/".to_string();
    full_file_path.push_str(filename);
    let input_file = File::open(full_file_path).unwrap();
    let input_reader = BufReader::new(input_file);

    // Parse the sequence from the FASTA file
    let mut sequence = String::new();
    
    
    for line in input_reader.lines() {
        let line = line.unwrap();
        if !line.starts_with('>') {
            sequence.push_str(&line);
        }
    }

    let sequence = sequence.to_uppercase();
    let current_suffix = Instant::now();
    let suffix_table = SuffixTable::new(sequence.clone());
    let suffix_array = suffix_table.table();
    let duration = current_suffix.elapsed();
    println!("Time elapsed in suffix_table() is: {:?}", duration);

    let mut sa_vec = Vec::<usize>::new();
    for val in suffix_array{
        sa_vec.push(*val as usize);
    }

    let mut prefix_table:Option<HashMap<String, Option<(i32,i32)>>> = None;
    if bin_p{
        let current_possible_prefix = Instant::now();
        let hs = build_all_prefixes(prefix_k.unwrap());
        let duration = current_possible_prefix.elapsed();
        println!("Time elapsed in build_all_prefixes() is: {:?}", duration); 
        let current_prefix_table = Instant::now();
        prefix_table =  Some(build_prefix(&suffix_table, hs.clone()));
        let duration = current_prefix_table.elapsed();
        // println!("{:?}", prefix_table.clone().unwrap());
        // println!("{}", "Here");
        println!("Time elapsed in build_prefix() is: {:?}", duration);
        // println!("{:?}", &suffix_table);
        // println!("{:?}", &hs);
        // println!("{:?}", &prefix_table);
        // println!("{:?}", &prefix_table.keys());
        
    }
    
    let data1 = Data {
        full_sequence: sequence,
        suffix: sa_vec.clone(),
        prefix_tab: prefix_table,
        size_of_suffix: sa_vec.clone().len(),
        prefix_len: prefix_k
    };
    let mut outfilepath = "src/index_files/".to_string();
    outfilepath.push_str(binary);
    let file_2 = File::create(outfilepath).unwrap();
    let mut bin_writer = BufWriter::new(file_2);
    let _res = bincode::serialize_into(&mut bin_writer, &data1);
    
}


fn build_all_prefixes(k: usize)->Vec<String>{

    let mut prefixes:Vec<String> = Vec::new();
    pub trait BuildPrefixes: Clone + Iterator
    where Self::Item: Clone {
    fn build_prefixes(self, k: usize) -> MultiProduct<Self> {
        std::iter::repeat(self).take(k).multi_cartesian_product()
    }}
    //Implementing the clone trait for Building the prefix
    impl<T: Iterator + Clone> BuildPrefixes for T
    where T::Item: Clone {}
    let pref: Vec<_>  = "ATCG".chars().collect();
    let pref_iter = pref.iter().build_prefixes(k);
    for j in pref_iter{
        let temp_pref = j.iter().join("");
        prefixes.push(temp_pref.clone());
    }
    prefixes
}

fn build_prefix(suffix_table: &SuffixTable, possible_pref:Vec<String>)-> HashMap<String, Option<(i32,i32)>>{
    let mut prefix_hash: HashMap<String, Option<(i32,i32)>>= HashMap::new();
    for i in &possible_pref{
        prefix_hash.insert(i.clone(), None);
    }
    let n3 = possible_pref[0].len();
    let n = suffix_table.len();
    for i in 0..n{
        let temp = suffix_table.suffix(i);
        if temp.len()<n3{
            continue;
        }else{
            let slice = &temp[0..n3].to_string();
            if let Some(Some(v1)) = prefix_hash.get(slice) {
                prefix_hash.insert(slice.clone(), Some((v1.0,v1.1+1)));
            }else{
                prefix_hash.insert(slice.clone(), Some((i as i32,i as i32 +1 )));
            }
        }
    }
    prefix_hash
}


