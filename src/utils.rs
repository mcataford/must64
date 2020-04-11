use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

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

pub fn split_fragment(fragment: &str) -> regex::Captures {
    let re = Regex::new(
        r"^(?P<op>\w+) (?:(?P<rd>\$\w+),) (?:(?P<rs>\$\w+),)? ?(?:(?P<rt>\$\w+)|(?P<imm>\d+))$",
    )
    .expect("Invalid regex");
    return re.captures(fragment).unwrap();
}
