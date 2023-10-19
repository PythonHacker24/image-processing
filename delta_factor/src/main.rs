use std::error::Error;
use std::fs::File;
use std::io;

fn extract_data(dataset_file_path: String) -> Result<Vec<Vec<i32>>, Box<dyn Error>> {
    let file = File::open(dataset_file_path)?;
    let reader = io::BufReader::new(file);

    let mut output_vector: Vec<Vec<i32>> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let mut numbers: Vec<i32> = Vec::new();

        for num_str in line.split(',') {
            if let Ok(num) = num_str.trim().parse() {
                numbers.push(num);
            }
        }
        output_vector.push(numbers);
    }
    Ok(output_vector)
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        println!("[*] Path to dataset not provided");
        std::process::exit(1);
    }

    match extract_data(args[0].clone()) {
        Ok(vertical_vector) => {
            for horizontal_vector in vertical_vector {
                for element in horizontal_vector {
                    print!("{},", element);
                }
                print!("\n");
            }
        }
        Err(error) => println!("Error in processing: {}", error),
    }
}
