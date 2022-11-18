use std::error::Error;
use std::fmt::Display;
use std::{io, thread};
use std::fs::{self, DirEntry};
use std::io::ErrorKind;
use std::ops::Range;
use std::path::{Path, PathBuf};
use std::sync::{Arc, mpsc, Mutex};
use std::thread::available_parallelism;
use std::time::Duration;
use regex::bytes::Regex;
use crate::GrepResult;

pub fn print_with_channel(all_files: &Vec<PathBuf>, regex: &Regex){
    let (tx, rx) = mpsc::channel();

    thread::spawn(move||{
        loop{
            match rx.recv() {
                Ok(grep_res) => println!("{}", grep_res), // printout the result
                Err(_) => return, // return for killing a thread
            }
        }
    });

    let core_num = available_parallelism().unwrap().get();

    let counter = Arc::new(Mutex::new(0));

    for i in (0..all_files.len()).step_by(core_num) {
        let mut thread_vec = Vec::new();
        for j in i..i+core_num {
            if j >= all_files.len() {
                // the sleep here is necessary, prevents the main function to end too fast killing all the undone threads
                std::thread::sleep(Duration::from_millis(1));
                return;
            }
            let path = all_files[j].clone();
            let regex = regex.clone();
            let counter = counter.clone();
            let tx = tx.clone();

            let search_thread = std::thread::spawn(move||{
                let gr = search(&path, &regex, counter);
                match gr {
                    Err(_) => (),
                    Ok(res) => {tx.send(res).unwrap();}
                }
            });

            //search_thread.join().unwrap();
            thread_vec.push(search_thread);
        }
        for thread in thread_vec {
            thread.join().unwrap();
        }
    }
}

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
pub fn search(path: &PathBuf, regex: &Regex, counter: Arc<Mutex<i32>>) -> Result<GrepResult, ErrorKind>{
    //println!("aaa");
    let mut ranges: Vec<Range<usize>>= Vec::new();
    let content = fs::read(path);
    match content {
        Err(error) => Err(ErrorKind::Interrupted),
        Ok(content) => {
            if regex.is_match(&content) {

                let mut grep_res = GrepResult {
                    path: path.clone(),
                    content: content.to_vec(),
                    ranges,
                    search_ctr: *counter.lock().unwrap() as usize,
                };
                *counter.lock().unwrap() += 1;
                for mat in regex.find_iter(&grep_res.content) {
                    grep_res.ranges.push(Range { start: mat.start(), end: mat.end() });
                }
                Ok(grep_res)
            }
            else { Err(ErrorKind::InvalidData) }
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
        let t = thread::spawn(move || {
            let gr = search(&path, &regex, c);
            match gr {
                Err(_) => (),
                Ok(res) => println!("{}", res),
            }
        });
        t.join().unwrap();
    }
    else {
        for i in 0..loop_times{
            let (c1, c2) = (counter.clone(), counter.clone());
            let path1 = entries[i * 2].clone();
            let path2 = entries[i * 2 + 1].clone();
            let regex1 = regex.clone();
            let regex2 = regex.clone();
            let t1 = thread::spawn(move || {
                let gr = search(&path1, &regex1, c1);
                match gr {
                    Err(_) => (),
                    Ok(res) => println!("{}", res),
                }
            });
            let t2 = thread::spawn(move || {
                let a = search(&path2, &regex2, c2);
                match a {
                    Err(_) => (),
                    Ok(res) => println!("{}", res),
                }
            });
            t1.join().unwrap();
            t2.join().unwrap();
        }

        if loop_left != 0 {
            let c = counter.clone();
            let path = entries[entries.len() - 1].clone();
            let regex = regex.clone();
            let t = thread::spawn(move || {
                let a = search(&path, &regex, c);
                match a {
                    Err(_) => (),
                    Ok(res) => println!("{}", res),
                }
            });
            t.join().unwrap();
        }
    }

}