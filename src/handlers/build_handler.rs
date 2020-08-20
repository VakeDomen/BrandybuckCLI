use crate::helpers::file_helper::{generate_folder, write_file, read_config_file, read_model_file};
use crate::models::node_package::NodePackage;
use crate::models::config_file::ConfigFile;
use crate::models::models_file::ModelFile;
use crate::models::ts_compiler_options::TsCompilerOptions;
use crate::helpers::migration_sqlite::generate_sqlite_migation_files;

pub fn build_application() -> () {
    let config_file: ConfigFile = read_config_file();
    let models_file: ModelFile = read_model_file(config_file.model_source.clone());
    generate_folder_structure(&config_file);
    generate_node_package_json(&config_file);
    generate_ts_config();
    generate_migration(&config_file, &models_file);
    
}
fn generate_ts_config() -> () {
    let ts_config = TsCompilerOptions::new();
    let mut serialized = serde_json::to_string_pretty(&ts_config).unwrap();
    write_file(&mut serialized, "app/tsconfig.json".to_string());
}

fn generate_migration(config: &ConfigFile, models: &ModelFile) -> () {
    if config.database == String::from("sqlite") {
        generate_sqlite_migation_files(config, models);
    }
}

fn generate_node_package_json(config: &ConfigFile) -> () {
    let package_json = NodePackage::new(config);
    let mut serialized = serde_json::to_string_pretty(&package_json).unwrap();
    write_file(&mut serialized, "app/package.json".to_string());
}

fn generate_folder_structure(config: &ConfigFile) -> () {
    println!("Generating folders...");
    generate_folder(String::from("./app/migrations"));
    generate_folder(String::from("./app/src/models"));
    generate_folder(String::from("./app/src/models/core"));
    generate_folder(String::from("./app/src/routes"));
    generate_folder(String::from("./app/src/db"));
    if config.auth {
        generate_folder(String::from("./app/src/auth"));
    }
    println!("Done generating folders");
}