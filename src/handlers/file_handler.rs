use std::fs;
use std::fs::File;
use std::io::prelude::*;

pub fn generate_folder(dir: String) -> () {
    let result = fs::create_dir_all(dir.clone());
    match result {
        Ok(_) => println!("Generated folder: {}", dir),
        Err(e) => eprintln!("Oops an error occured while generating folder {}: {}", dir, e)
    }
}

pub fn copy_file(source: String, destination: String) -> () {
    let result = fs::copy(source.clone(), destination.clone());
    match result {
        Ok(_) => println!("Copied file {} to {}", source, destination),
        Err(e) => eprintln!("Oops an error occured while trying to copy file from {} to {}: {}", source, destination, e)
    }
}

pub fn write_file(string: &mut String, destination: String) -> () {
    let file = File::create(destination.clone());
    match file {
        Ok(mut f) => {
            println!("Created file {}.", destination);
            let result = f.write_all( unsafe { string.as_bytes_mut() });
            match result {
                Ok(_) => println!("Wrote file {}", destination),
                Err(e) => eprintln!("Oops an error occured while trying to write to file file {}: {}", destination, e)
            }
        },
        Err(e) => eprintln!("Oops an error occured while trying to create file {}: {}", destination, e)
    }
}