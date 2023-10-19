extern crate image; 

use std::{env, fs::{self, File}, error::Error, io::{self, BufRead}};
use image::GenericImageView;

fn image_to_vector(location: String) -> Vec<Vec<Vec<u8>>>{
    let img = image::open(location).unwrap();
    let (width, height) = img.dimensions();
    
    let mut output: Vec<Vec<Vec<u8>>> = Vec::new();
    for height_element in 0..height {
        let mut horizontal_vector: Vec<Vec<u8>> = Vec::new();
        for width_element in 0..width {
            let mut rgb_vector: Vec<u8> = Vec::new();
            let pixel = img.get_pixel(width_element, height_element);
            rgb_vector.push(pixel[0]);
            rgb_vector.push(pixel[1]);
            rgb_vector.push(pixel[2]);
            horizontal_vector.push(rgb_vector);
        }
        output.push(horizontal_vector);
    }

    return output;

}

fn r_pixelator(image_vector: Vec<Vec<Vec<u8>>>) -> Vec<Vec<u8>> {
    let mut output: Vec<Vec<u8>> = Vec::new();
    for horizontal_vector in image_vector {
        let mut output_horizontal_vector: Vec<u8> = Vec::new();
        for rgb_vector in horizontal_vector {
            output_horizontal_vector.push(rgb_vector[0]);
        }
        output.push(output_horizontal_vector);
    }
    return output;
}

fn linearator(pixelated_vector: Vec<Vec<u8>>) -> Vec<u8> {
    let mut output: Vec<u8> = Vec::new();
    for horizontal_vector in pixelated_vector {
        for pixel in horizontal_vector {
            output.push(pixel);
        }
    }
    return output;
}

fn linear_data_of_image(path: String) -> Vec<u8> {
    let image_vector: Vec<Vec<Vec<u8>>> = image_to_vector(path);
    let pixelated_vector: Vec<Vec<u8>> = r_pixelator(image_vector);
    let linear_vector: Vec<u8> = linearator(pixelated_vector);
    
    return linear_vector;
}

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
    
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        println!("[*] Path to image not provided");
        std::process::exit(1);
    }

    let image_path = args[0].clone();
    let dataset_path = args[1].clone();

    let mut dataset_vector: Vec<Vec<i32>> = Vec::new();
    match extract_data(dataset_path) {
        Ok(extracted_vector) => {
            dataset_vector = extracted_vector;
        }
        Err(error) => println!("Error in processing dataset: {}", error),
    }

    let input_image_linear_vector = linear_data_of_image(image_path);
    let mut delta_vector: Vec<Vec<i32>> = Vec::new();
    for sample_vector in dataset_vector {
        let delta_layer: Vec<i32> = Vec::new();
        for pixel_pointer in 0..=sample_vector.len() {
            delta_vector.push(dataset_vector[pixel_pointer] - input_image_linear_vector[pixel_pointer]);
        }
        delta_vector.push(delta_layer);
    }

    for horizontal_vector in delta_vector {
        for pixel in horizontal_vector {
            print!("{} ", pixel);
        }
        print!("\n");
    }
}

