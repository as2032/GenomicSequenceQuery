use std::fs::File;
use std::io::{BufRead, BufReader};
use std::cmp;
use std::env;
use std::io::prelude::*;
use bincode;
use std::collections::HashMap;
use std::time::{Instant};
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
    let index = &args[1];
    let query_file = &args[2];
    let q_mode = &args[3];
    let output = &args[4];
    let mut ptb = false;
    let mut binfile = "src/index_files/".to_string();
    binfile.push_str(index);
    let file = File::open(binfile).unwrap();
    let input_reader = BufReader::new(file);
    let data: Data = bincode::deserialize_from(input_reader).unwrap();
    if !data.prefix_len.is_none(){
        ptb = true;
    }
    let mut fasta_files = "input_files/query_files/".to_string();
    fasta_files.push_str(query_file);
    let query_input_file = File::open(fasta_files).unwrap();
    let input_reader = BufReader::new(query_input_file);
    let mut seq = String::new();
    let mut query_names = Vec::new();
    let mut queries = Vec::new();
    let mut q_count = 0;
    for line in input_reader.lines() {
        let line = line.unwrap();
        if line.starts_with('>') {
            query_names.push(line.clone());
            if q_count!=0 {
                seq = seq.to_uppercase();
                queries.push(seq.clone());
                seq = String::new();
            }
            q_count = q_count +1;
        }else{

            seq.push_str(&line);
        }
    }
    queries.push(seq.clone());
    let mut s_quer = Vec::new();

    for qn in &query_names {
        let name: Vec<&str> = qn.split(' ').collect();
        let mut temp = name[0];
        temp = &temp[1..];
        s_quer.push(temp.to_string());
    }
    query_names = s_quer;
    let mut responses= Vec::new();
    let mut q_i = 0;
    let prefix_table_un = data.prefix_tab.unwrap();
    if q_mode == "naive"{
        for target_q in &queries{
            let mut start = 0;
            let mut end = data.full_sequence.clone().len();
            let mut invalid_t = false;
            if ptb{
                if &target_q.len()<&data.prefix_len.unwrap(){
                    invalid_t = true;
                }else{
                    if let Some((start1,end1))=  prefix_table_un[&target_q.clone()[0..data.prefix_len.unwrap()]]{
                        start = start1 as usize;
                        end = end1 as usize;
                    }else{
                        invalid_t = true;
                    }
                }
            }
            if !invalid_t{
                let current_bins = Instant::now();
                let bins = binary_search_vanilla(data.suffix.clone(), &target_q.clone(), &data.full_sequence.clone(), start, end);
                let duration = current_bins.elapsed();
                println!("Time elapsed in binary_search_vanilla() for query: {} is: {:?}", &query_names[q_i], duration); 
                responses.push(bins);
            }else{
                println!("Query: {} was not queried since length < prefix length", &query_names[q_i]); 
                responses.push(Vec::new());
           }
           q_i+=1;
        }
    }else if q_mode=="simpaccel"{
        for target_q in &queries{
           
            let mut start = 0;
            let mut end = data.full_sequence.clone().len();
            let mut invalid_t = false;
            if ptb{
                if &target_q.len()<&data.prefix_len.unwrap(){
                    invalid_t = true;
                    println!("here");
                }else{
                    if let Some((start1,end1)) =  prefix_table_un[&target_q.clone()[0..data.prefix_len.unwrap()]]{
                        start = start1 as usize;
                        end = end1 as usize;
                    }else{
                        invalid_t = true;
                    }
                    
                }
            }
            if !invalid_t{
                let current_bins = Instant::now();
                let bins = binary_search_accel(data.suffix.clone(), &target_q.clone(), &data.full_sequence.clone(), start, end);
                let duration = current_bins.elapsed();
                println!("Time elapsed in binary_search_accel() for query: {} is: {:?}", &query_names[q_i], duration); 
                responses.push(bins);
            }else{
                println!("Query: {} was not queried since length < prefix length", &query_names[q_i]); 
                responses.push(Vec::new());
           }
           q_i+=1;
        }
    }else{
        println!("Please select either 'naive' or 'simpaccel' query mode");
    }
    if responses.len()>0{
        let mut outfilepath = "output_files/".to_string();
        outfilepath.push_str(output);
        let mut out_file = File::create(outfilepath).expect("Create Failed");
        for i in 0..queries.len(){
            out_file.write_all(query_names[i].as_bytes()).expect("WF");
            out_file.write_all("    ".as_bytes()).expect("WF");
            let k = responses[i].len() as u32;
            let resp_cpy = responses.clone();
            out_file.write_all(k.to_string().as_bytes()).expect("WF");
            for j in &resp_cpy[i]{
                out_file.write_all("    ".as_bytes()).expect("WF");
                out_file.write_all(j.to_string().as_bytes()).expect("WF");
            }
            out_file.write_all("\n".as_bytes()).expect("WF")
    }
    }
}




pub fn binary_search_vanilla<'a>(array:Vec<usize>, target: &'a str, sequence: &'a str, mut low: usize, mut hi: usize)-> Vec<usize>{
    let mut result = Vec::new();
    hi = hi-1;
    while low<hi{
        let m = (low+hi)/2;
        if &sequence[array[m]..array[m]+&target.len()] < target{
            low = m+1;
        }else if  &sequence[array[m]..array[m]+&target.len()]> target{
            hi = m-1;
        }else{
            if sequence[array[m]..].eq(target){
                            // println!("{}", &sequence[array[m]..]);
                            // println!("{}", target);
                            // println!("{}", array[m]);
                            result.push(array[m]);
            }
            let mut lft = m-1;
            while lft>=low{
                let temp = &sequence[array[lft]..];
                if !temp.starts_with(target){
                    break;
                }
                // println!("{}", &sequence[array[lft]..]);
                // println!("{}", target);
                // println!("{}", array[lft]);
                result.push(array[lft]);
                lft-=1;
            }
            let mut rgt = m+1;
            while rgt<=hi{
                let temp = &sequence[array[rgt]..];
                if !temp.starts_with(target){
                    break;
                }
                // println!("{}", &sequence[array[rgt]..]);
                // println!("{}", target);
                // println!("{}", array[rgt]);
                result.push(array[rgt]);
                rgt+=1;
            }
            break;
        }
    }
result
}

pub fn compute_lcp(seq:&str, target: &str)->usize{
    let mut i:usize = 0;
    while i < target.len() && i<seq.len(){
        if target.as_bytes()[i]!=seq.as_bytes()[i]{
            break;
        }
        i+=1;
    }
    i
}

pub fn binary_search_accel<'a>(array:Vec<usize>, target: &'a str, sequence: &'a str, mut low: usize, mut hi: usize)-> Vec<usize>{

    let mut result = Vec::new();
    hi = hi-1;
    //When compare c with center suffix, we dont start at beginning of strings we start at min lcp pl lcp pr
    let mut lcp_pl = compute_lcp(&target, &sequence[array[low]..]);
    let mut m = (low+hi)/2;
    // let mut lcp_pm = compute_lcp(&target, &sequence[array[m]..]);
    let mut lcp_pr = compute_lcp(&target, &sequence[array[hi]..]);
    while low<hi{
        m = (low+hi)/2;
        let minlcp = cmp::min(lcp_pl,lcp_pr);
        let mut j = minlcp;
        while j<target.len(){
            if sequence.as_bytes()[array[m]+j]<target.as_bytes()[j]{
                //bisect right
                lcp_pl = j;
                low = m+1;
                break
            }else if sequence.as_bytes()[array[m]+j]>target.as_bytes()[j]{
                //bisect left
                lcp_pr = j;
                hi = m-1;
                break
            }
            j= j+ 1;
        }
        if j == target.len(){
            break
        }
    }
           
    if sequence[array[m]..].eq(target){
        result.push(array[m]);
    }
    let mut lft = m-1;
    while lft>=low{
        let temp = &sequence[array[lft]..];
        if !temp.starts_with(target){
            break;
        }
        // println!("{}", &sequence[array[lft]..]);
        // println!("{}", target);
        // println!("{}", array[lft]);
        result.push(array[lft]);
        lft-=1;
    }
    let mut rgt = m+1;
    while rgt<=hi{
        let temp = &sequence[array[rgt]..];
        if !temp.starts_with(target){
            break;
        }
        // println!("{}", &sequence[array[rgt]..]);
        // println!("{}", target);
        // println!("{}", array[rgt]);
        result.push(array[rgt]);
        rgt+=1;
    }
    result         
    }
            
            
            
            
        
    














