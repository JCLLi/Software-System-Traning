mod test;

use clap::Parser;
use regex::bytes::Regex;
use std::fmt::{Display, Formatter};
use std::num::NonZeroUsize;
use std::ops::Range;
use std::path::PathBuf;
use std::{fs, thread};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The file/folder name filter
    #[arg(short, long)]
    filter: Option<String>,

    /// The regex pattern that the user provided
    regex: String,

    /// The paths in which mygrep should search, if empty, in the current directory
    paths: Vec<String>,
}

fn main() {
    //Parse arguments, using the clap crate
    let args: Args = Args::parse();
    let regex = Regex::new(&args.regex).unwrap();

    // Get the paths that we should search
    let paths = if args.paths.is_empty() {
        //If no paths were provided, we search the current path
        vec![std::env::current_dir().unwrap()]
    } else {
        // Take all paths from the command line arguments, and map the paths to create PathBufs
        args.paths.iter().map(PathBuf::from).collect()
    };
    // println!("length: {}", paths.len());
    // let a = paths[0].clone();
    // let b = paths[1].clone();
    //
    // println!("{}", a.to_str().unwrap());
    // println!("{}", b.to_str().unwrap());
    let mut all_files: Vec<PathBuf> = Vec::new();
    for i in 0..paths.len(){
        test::ite(&mut all_files, &paths[i]);
    }
    test::search(&mut all_files, regex);

}

/// This structure represents the matches that the tool found in **a single file**.
/// It implements `Display`, so it can be pretty-printed.
/// This struct and the `Display` trait implementation do NOT need to be edited.
struct GrepResult {
    /// the path of the search result
    path: PathBuf,

    /// the contents of that file
    content: Vec<u8>,

    /// which ranges in the file match the filter.
    /// A file may contain more than one match. Each match is a Range,
    /// which is a start and end byte offset in the content field.
    ranges: Vec<Range<usize>>,

    /// The index of this search result (ie. a counter of how many files have had a match before this
    /// one). Note that this count must always increase as the results are printed.
    search_ctr: usize,
}

impl Display for GrepResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let max_context = 70;

        if self.ranges.is_empty() {
            return Ok(());
        }

        writeln!(f, ">>> (#{}) {:?}", self.search_ctr, self.path)?;
        for range in &self.ranges {
            // Find the index of the first byte before the range that is a newline character, plus one.
            let mut ctx_start = (0..range.start)
                .rev()
                .find(|p| self.content[*p] == b'\n' || self.content[*p] == b'\r')
                .map(|p| p + 1)
                .unwrap_or(0);

            // Find the index of the first byte after the range that is a newline character (not minus one because it is exclusive)
            let mut ctx_end = (range.end..self.content.len())
                .find(|p| self.content[*p] == b'\n' || self.content[*p] == b'\r')
                .unwrap_or(self.content.len());

            // if the context is too large, reduce its size
            if ctx_start + max_context < range.start {
                ctx_start = range.start - max_context;
            }
            if ctx_end > range.end + max_context {
                ctx_end = range.end + max_context;
            }

            // Finally, print the result
            writeln!(
                f,
                "{}",
                String::from_utf8_lossy(&self.content[ctx_start..ctx_end])
            )?;
            // Print ^^^^ underneath matched part
            writeln!(
                f,
                "{}{}{}",
                " ".repeat(range.start - ctx_start),
                "^".repeat(range.end - range.start),
                " ".repeat(ctx_end - range.end)
            )?;
        }

        Ok(())
    }
}
