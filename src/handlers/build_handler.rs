use crate::helpers::file_helper::{generate_folder, write_file, read_config_file, read_model_file};
use crate::models::node_package::NodePackage;
use crate::models::config_file::ConfigFile;
use crate::models::models_file::ModelFile;

pub fn build_application() -> () {
    let config_file: ConfigFile = read_config_file();
    let models_file: ModelFile = read_model_file(config_file.model_source.clone());
    generate_folder_structure();
    generate_node_package_json(&config_file);
    generate_migration(&config_file);
}

pub fn generate_migration(config: &ConfigFile) -> () {

}

fn generate_node_package_json(config: &ConfigFile) -> () {
    let package_json = NodePackage::new(config);
    let mut serialized = serde_json::to_string_pretty(&package_json).unwrap();
    write_file(&mut serialized, "app/package.json".to_string());
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