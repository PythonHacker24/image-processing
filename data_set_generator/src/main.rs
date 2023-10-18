extern crate image; 

use std::{env, fs};
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

fn process_images_in_directory(input_dir: String, database_vec: &mut Vec<Vec<u8>>) -> Result<(), Box<dyn std::error::Error>> {
    for entry in fs::read_dir(input_dir)? {
        let entry = entry?;
        let path = entry.path();

        if let Some(extension) = path.extension() {
            if extension == "jpg" {
                let string_path = path.to_string_lossy().to_string();
                let linear_vector = linear_data_of_image(string_path);
                database_vec.push(linear_vector);
            }
        }
    }
    Ok(())
}

fn main() {
    
    // Path: Path/{number}
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        println!("[*] Path to image not provided!");
        std::process::exit(1);
    }
    
    let mut database_vec: Vec<Vec<u8>> = Vec::new();
    let _ = process_images_in_directory(args[0].clone(), &mut database_vec);
    for horizontal_vector in database_vec {
        for pixel in horizontal_vector {
            print!("{},", pixel);
        }
        print!("\n");
    } 
}

// Notes //  
// The dataset that has been provided has the values of r, g, and b equal.
// consider only r value for the pixel
