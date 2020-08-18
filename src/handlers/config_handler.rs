use crate::models::cli::CliOptions;
use crate::models::config_file::ConfigFile;
use crate::handlers::file_handler::{generate_folder, write_file};

pub fn generate_new_configurations(options: CliOptions) -> () {
    generate_model_template(&options);
    generate_configuration_template(&options);
}

fn generate_configuration_template(_options: &CliOptions) -> () {
    let configuration = ConfigFile::new();
    let mut serialized = serde_json::to_string_pretty(&configuration).unwrap();
    write_file(
        &mut serialized,
        String::from("./brandybuck.config.json")
    );
}

fn generate_model_template(_options: &CliOptions) -> () {
    
}

fn generate_folder_structure() -> () {
    println!("Generating folders...");
    generate_folder(String::from("./app/migrations"));
    generate_folder(String::from("./app/src/models"));
    generate_folder(String::from("./app/src/models/core"));
    generate_folder(String::from("./app/src/routes"));
    generate_folder(String::from("./app/src/db"));
    generate_folder(String::from("./app/src/auth"));
    println!("Done generating folders");
}