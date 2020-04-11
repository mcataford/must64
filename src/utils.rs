use std::io::BufReader;
use std::fs::File;
use std::io::prelude::*;
use regex::Regex;


pub fn read_source(path: &str) -> Vec<String> {
    let src_file = File::open(path).expect("Cannot read file");
    let src_content = BufReader::new(src_file);
    let mut fragments = Vec::new();


    for line in src_content.lines() {
        let line = line.unwrap();
        fragments.push(line);
    }

    return fragments;
}

pub fn split_fragment(fragment: &str) -> Vec<&str> {
    let re = Regex::new(r"[\s,]+").expect("Invalid regexp");
    let parts: Vec<_> = re.split(fragment).into_iter().collect();
    return parts
}

