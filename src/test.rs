use crate::GrepResult;
use regex::bytes::Regex;
use std::fs::{self};
use std::io::ErrorKind;
use std::ops::Range;
use std::path::{Path, PathBuf};
use std::sync::{mpsc, Arc, Mutex};
use std::thread::available_parallelism;
use std::time::Duration;
use std::{io, thread};

pub fn print_with_channel(all_files: &Vec<PathBuf>, regex: &Regex) {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        loop {
            match rx.recv() {
                Ok(grep_res) => println!("{}", grep_res), // printout the result
                Err(_) => return,                         // return for killing a thread
            }
        }
    });

    let core_num = available_parallelism().unwrap().get();

    let counter = Arc::new(Mutex::new(0));

    for i in (0..all_files.len()).step_by(core_num) {
        let mut thread_vec = Vec::new();
        for j in i..i + core_num {
            if j >= all_files.len() {
                // the sleep here is necessary, prevents the main function to end too fast killing all the undone threads
                std::thread::sleep(Duration::from_millis(1));
                return;
            }
            let path = all_files[j].clone();
            let regex = regex.clone();
            let counter = counter.clone();
            let tx = tx.clone();

            let search_thread = std::thread::spawn(move || {
                let grep_result = search(&path, &regex, counter);
                match grep_result {
                    Err(error) => {
                        if error == ErrorKind::InvalidInput {
                            panic!("Can not read the file")
                        }
                    }
                    Ok(res) => {
                        tx.send(res).unwrap();
                    }
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

pub fn ite(entries: &mut Vec<PathBuf>, path: &Path) -> io::Result<()> {
    let mut temp = fs::read_dir(path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    temp.sort();
    for path in &temp {
        if path.is_dir() {
            let res = ite(entries, path);
            match res {
                Ok(_) => (),
                Err(err) => panic!("This is not a valid path, error is {}", err),
            }
        } else {
            entries.push(path.clone());
        }
    }
    Ok(())
}
pub fn search(
    path: &PathBuf,
    regex: &Regex,
    counter: Arc<Mutex<i32>>,
) -> Result<GrepResult, ErrorKind> {
    let ranges: Vec<Range<usize>> = Vec::new();
    let content = fs::read(path);
    match content {
        Err(_) => Err(ErrorKind::InvalidInput),
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
                    grep_res.ranges.push(Range {
                        start: mat.start(),
                        end: mat.end(),
                    });
                }
                Ok(grep_res)
            } else {
                Err(ErrorKind::InvalidData)
            }
        }
    }
}
