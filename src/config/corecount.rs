use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum ThreadCount {
    /// use all cores
    #[serde(rename = "all")]
    All,

    /// Number of threads to be used
    #[serde(rename = "count")]
    Count(usize),

    /// Number of threads left over when running
    #[serde(rename = "left")]
    Left(usize),
}

impl ThreadCount {
    pub fn get_cores(&self) -> usize {
        let num_cpus = std::thread::available_parallelism().unwrap().into();

        match self {
            ThreadCount::Left(threads) => num_cpus - *threads,
            ThreadCount::Count(threads) => *threads,
            ThreadCount::All => num_cpus,
        }
    }
}

impl Default for ThreadCount {
    fn default() -> Self {
        ThreadCount::All
    }
}
