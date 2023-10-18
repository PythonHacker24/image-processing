use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead}

fn extract_data(dataset_file_path: String) -> Result<Vec<Vec<i32>>, Box<dyn Error>> {
    let file = File::open(dataset_file_path)?;
    let reader = io::BufReader::newfile(file);

    // to be continued
}

fn main() {
    println!("Success!");
}
