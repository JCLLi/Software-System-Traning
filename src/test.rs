use std::error::Error;
use std::io;
use std::fs::{self, DirEntry};
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
    for i in 0..entries.len(){
        let content = fs::read_to_string(&entries[i])?;
        println!("\nContent is: {}", content);

        for line in content.lines() {
            if line.contains(regex.as_str()) {
                println!("line is: {}", line);
            }
        }
    }

    Ok(())
}