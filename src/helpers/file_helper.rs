use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use crate::models::brandybuck_config_file::ConfigFile;
use crate::models::brandybuck_models_file::ModelFile;

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

pub fn read_file(filepath: String) -> String {
    let file = File::open(filepath.to_string())
        .expect("could not open file");
    let mut buffered_reader = BufReader::new(file);
    let mut contents = String::new();
    let _number_of_bytes: usize = match buffered_reader.read_to_string(&mut contents) {
        Ok(number_of_bytes) => number_of_bytes,
        Err(_err) => 0
    };
    contents
}

pub fn read_model_file(filepath: String) -> ModelFile {
    let config_json = read_file(filepath.clone());
    let config_file = serde_json::from_str(&config_json);
    match config_file {
        Ok(file) => file,
        Err(e) => { 
            println!("Error reading model file: {}", e);
            panic!("Error loading a {}. If there is none, consider using the 'init' command", filepath);
        },
    }
}

pub fn read_config_file() -> ConfigFile {
    let config_json = read_file(String::from("./brandybuck.config.json"));
    let config_file = serde_json::from_str(&config_json);
    match config_file {
        Ok(file) => file,
        Err(e) => { 
            println!("Error reading config file: {}", e);
            panic!("Error loading a brandybuck.config.json. If there is none, consider using the 'init' command");
        },
    }
}