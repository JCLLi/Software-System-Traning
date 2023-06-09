use crate::GrepResult;
use regex::bytes::Regex;
use std::fs::{self};
use std::io::ErrorKind;
use std::ops::Range;
use std::path::{Path, PathBuf};
use std::process::exit;
use std::sync::{mpsc, Arc, Mutex};
use std::thread::available_parallelism;
use std::time::Duration;
use std::{io, thread};


/// Print the every result which matches the regex.
///
/// # Arguments
///
/// * `all_files` - a vector with all file paths in it.
/// * `regex` - the regex needs to be searched in files.
///
/// # Return
/// Nothing is returned
pub fn print_with_channel(all_files: &Vec<PathBuf>, regex: &Regex) {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        loop {
            match rx.recv() {
                Ok(grep_res) => println!("{}", grep_res), // printout the result
                Err(_) => {
                    println!("\nChannel closed, program finished\n");
                    exit(0);
                } // return for killing a thread
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
                break;
            }
            let path = all_files[j].clone();
            let regex = regex.clone();
            let counter = counter.clone();
            let tx = tx.clone();

            let search_thread = std::thread::spawn(move || {
                let grep_result = regex_search(&path, &regex, counter);
                match grep_result {
                    Err(error) => {
                        if error == ErrorKind::InvalidInput {
                            println!("\n!!!Can not read the file, program is ended!!!\n");
                            exit(0);
                        }
                    }
                    Ok(res) => {
                        if let Err(err) = tx.send(res) {
                            println!("\n!!!Error happened with tx of channel, program is ended with error {}!!!\n", err);
                            exit(0);
                        }
                    }
                }
            });
            thread_vec.push(search_thread);
        }
        for thread in thread_vec {
            if thread.join().is_err() {
                println!("\n!!!Error happened with thread join, program is ended with error!!!\n");
                exit(0);
            }
        }
    }
}

/// Print the every result which matches the regex.
///
/// # Arguments
///
/// * `entries` - a vector with all file paths in it.
/// * `path` - the path input by users.
/// * `filter` - the keyword of filter.
///
/// # Return
/// * `io::Result<()>` - return IO errors when the path input by user is wrong.
pub fn find_path(
    entries: &mut Vec<PathBuf>,
    path: &Path,
    filter: &Option<String>,
) -> io::Result<()> {
    let mut path_set = fs::read_dir(path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    path_set.sort();

    match filter {
        Some(filter_content) => {
            for path in &path_set {
                if path.is_dir() {
                    let res = find_path(entries, path, filter);
                    match res {
                        Ok(_) => (),
                        Err(err) => {
                            return Err(err);
                        }
                    }
                } else {
                    let key = Regex::new(filter_content);
                    match key {
                        Ok(key) => match path.to_str() {
                            Some(unwrap_path) => {
                                if key.is_match(unwrap_path.as_bytes()) {
                                    entries.push(path.clone());
                                }
                            }
                            None => {
                                println!("\n!!!Invalid path, please check if the path is right, program is ended!!!\n");
                                exit(0);
                            }
                        },
                        Err(_) => {
                            println!("\n!!!Invalid filter keywords, please check if your paths are correct, program is ended!!!\n");
                            exit(0);
                        }
                    }
                }
            }
        }
        None => {
            for path in &path_set {
                if path.is_dir() {
                    let res = find_path(entries, path, filter);
                    match res {
                        Ok(_) => (),
                        Err(err) => {
                            return Err(err);
                        }
                    }
                } else {
                    entries.push(path.clone());
                }
            }
        }
    }
    Ok(())
}

/// Print the every result which matches the regex.
///
/// # Arguments
///
/// * `path` - one of the file paths in the aim folder.
/// * `regex` - the regex needs to be searched in files.
/// * `counter` - how many files with regex are found.
///
/// # Return
/// * `Result<GrepResult, ErrorKind>` - return errors when the file can not be read or the a
///    GrepResult when there are regex matches.
pub fn regex_search(
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
