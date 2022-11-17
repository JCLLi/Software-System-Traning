use std::error::Error;
use std::io;
use std::fs::{self, DirEntry};
use std::ops::Range;
use std::path::{Path, PathBuf};
use regex::bytes::Regex;
use crate::GrepResult;

pub fn ite(entries: &mut Vec<PathBuf>, path: &Path) -> io::Result<()>{
    let mut temp = fs::read_dir(path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    // The order in which `read_dir` returns entries is not guaranteed. If reproducible
    // ordering is required the entries should be explicitly sorted.

    temp.sort();
    for i in 0..temp.len(){
        if temp[i].is_dir(){
            ite(entries, &temp[i]);
        }else{
            entries.push(temp[i].clone());
        }

    }
    // The entries have now been sorted by their path.
    // for i in 0..entries.len(){
    //     println!("path: {}", entries[i].to_str().unwrap())
    // }


    Ok(())
}
pub fn search(entries: &mut Vec<PathBuf>, regex: Regex,) -> Result<(), Box<dyn Error>>{
    let mut counter = 0;
    for i in 0..entries.len(){
        let mut ranges: Vec<Range<usize>>= Vec::new();
        let content = fs::read_to_string(&entries[i])?;
        let contents_u8 = content.as_bytes();
        println!("There are {} characters in all", contents_u8.len());
        if regex.is_match(contents_u8){
            counter += 1;
            let mut grep_res = GrepResult{
                path: entries[i].clone(),
                content: contents_u8.to_vec(),
                ranges,
                search_ctr: counter,
            };
            for mat in regex.find_iter(contents_u8) {
                grep_res.ranges.push(Range{start: mat.start(), end: mat.end()});
            }
            for i in 0..grep_res.ranges.len(){
                println!("This match starts at {} and ends at {}", grep_res.ranges[i].start, grep_res.ranges[i].end);
            }



        }

    }

    Ok(())
}