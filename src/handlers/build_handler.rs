use crate::helpers::file_helper::{generate_folder, write_file, read_config_file, read_model_file};
use crate::models::brandybuck_config_file::ConfigFile;
use crate::models::brandybuck_models_file::ModelFile;
use crate::models::db_table_structure::DbTableStructure;
use crate::models::file_config_tssconfig_json::TsCompilerOptions;
use crate::models::file_config_package_json::NodePackage;
use crate::db_generators::sqlite::{generate_sqlite_migation_files, generate_orm_code};

pub fn build_application() -> () {
    let config_file: ConfigFile = read_config_file();
    let models_file: ModelFile = read_model_file(config_file.model_source.clone());
    generate_folder_structure(&config_file);
    generate_node_package_json(&config_file);
    generate_ts_config();
    generate_migration(&config_file, &models_file);
    generate_orm(&config_file);
    generate_table_config(&config_file, &models_file);
    
}

fn generate_table_config(config: &ConfigFile, models: &ModelFile) -> () {
    if config.database == String::from("sqlite") {
        let db_config = DbTableStructure::new(config, models);
        let mut serialized = serde_json::to_string_pretty(&db_config).unwrap();
        write_file(&mut serialized, "app/src/db/database.config.json".to_string());
    }
}

fn generate_orm(config: &ConfigFile) -> () {
    let mut orm_code = "".to_string();
    if config.database == String::from("sqlite") {
        orm_code = generate_orm_code();
    }
    write_file(&mut orm_code, "app/src/db/database.handler.ts".to_string());
}

fn generate_ts_config() -> () {
    let ts_config = TsCompilerOptions::new();
    let mut serialized = serde_json::to_string_pretty(&ts_config).unwrap();
    write_file(&mut serialized, "app/tsconfig.json".to_string());
}

fn generate_migration(config: &ConfigFile, models: &ModelFile) -> () {
    let mut migration_file = "".to_string();
    let mut file_name = "".to_string();
    if config.database == String::from("sqlite") {
        migration_file = generate_sqlite_migation_files(config, models);
        file_name = "app/migrations/001-inital-schema.sql".to_string();
    }
    write_file(&mut migration_file, file_name);
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