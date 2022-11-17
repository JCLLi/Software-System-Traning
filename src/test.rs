use std::error::Error;
use std::fmt::Display;
use std::{io, thread};
use std::fs::{self, DirEntry};
use std::ops::Range;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use regex::bytes::Regex;
use crate::GrepResult;

pub fn ite(entries: &mut Vec<PathBuf>, path: &Path) -> io::Result<()>{
    let mut temp = fs::read_dir(path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    temp.sort();
    for i in 0..temp.len(){
        if temp[i].is_dir(){
            ite(entries, &temp[i]);
        }else{
            entries.push(temp[i].clone());
        }

    }
    Ok(())
}

pub fn search(path: &PathBuf, regex: &Regex, counter: Arc<Mutex<i32>>){
    //println!("aaa");
    let mut ranges: Vec<Range<usize>>= Vec::new();
    let content = fs::read(path);
    match content {
        Err(error) => panic!("Problem reading the file: {:?}", error),
        Ok(content) => {
            if regex.is_match(&content) {
                *counter.lock().unwrap() += 1;
                let mut grep_res = GrepResult {
                    path: path.clone(),
                    content: content.to_vec(),
                    ranges,
                    search_ctr: *counter.lock().unwrap() as usize,
                };
                for mat in regex.find_iter(&grep_res.content) {
                    grep_res.ranges.push(Range { start: mat.start(), end: mat.end() });
                }
                println!("{}", grep_res);
            }
        }
    }
}

pub fn printout(entries: &Vec<PathBuf>, regex: Regex){
    let counter = Arc::new(Mutex::new(0));
    let loop_times = entries.len() / 2;
    let loop_left = entries.len() % 2;
    if loop_times == 0{
        let c = counter.clone();
        let path = entries[entries.len() - 1].clone();
        let regex = regex.clone();
        let t = thread::spawn(move || search(&path, &regex, c));
        t.join().unwrap();
    }
    else {
        for i in 0..loop_times{
            let (c1, c2) = (counter.clone(), counter.clone());
            let path1 = entries[i * 2].clone();
            let path2 = entries[i * 2 + 1].clone();
            let regex1 = regex.clone();
            let regex2 = regex.clone();
            let t1 = thread::spawn(move || search(&path1, &regex1, c1));
            let t2 = thread::spawn(move || search(&path2, &regex2, c2));
            t1.join().unwrap();
            t2.join().unwrap();
        }

        if loop_left != 0 {
            let c = counter.clone();
            let path = entries[entries.len() - 1].clone();
            let regex = regex.clone();
            let t = thread::spawn(move || search(&path, &regex, c));
            t.join().unwrap();
        }
    }

}
