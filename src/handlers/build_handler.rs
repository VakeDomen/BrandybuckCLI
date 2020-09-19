use crate::db_generators::sqlite::{generate_orm_code, generate_sqlite_migation_files};
use crate::helpers::file_helper::{generate_folder, read_config_file, read_model_file, write_file};
use crate::models::brandybuck_config_file::{ConfigFile, Docker};
use crate::models::brandybuck_models_file::{ModelFile};
use crate::models::db_table_structure::DbTableStructure;
use crate::models::file_config_package_json::NodePackage;
use crate::models::file_config_tssconfig_json::TsCompilerOptions;
use crate::ts_generators::file_server::generate_server_file;
use crate::ts_generators::files_models::{generate_core_models, generate_app_models};
use crate::ts_generators::files_dotenv::{generate_dotenv_file, generate_dotenv_sample_file};
use crate::ts_generators::files_routes::generate_routes_files;
use crate::ts_generators::files_auth::generate_auth_files;
use crate::db_generators::db_types::DbType;
use crate::docker_generators::files_docker::generate_docker_files;


pub fn build_application() -> () {
    let config_file: ConfigFile = read_config_file();
    let models_file: ModelFile = read_model_file(config_file.model_source.clone());
    generate_folder_structure(&config_file);
    generate_node_package_json(&config_file);
    generate_ts_config(&config_file);
    generate_migration(&config_file, &models_file);
    generate_orm(&config_file);
    generate_table_config(&config_file, &models_file);
    generate_server(&config_file, &models_file);
    generate_models(&config_file, &models_file);
    generate_dotenv(&config_file);
    generate_routes(&config_file, &models_file);
    if config_file.auth {
        generate_auth(&config_file);
    }
    generate_dockerization(&config_file);
    
}

fn generate_dockerization(config_file: &ConfigFile) -> () {
    match &config_file.docker {
        Docker::Bool(b) => if *b { generate_dockerization_files(config_file) },
        Docker::Config(conf) => generate_dockerization_files(config_file)
    }
    
}

fn generate_dockerization_files(config_file: &ConfigFile) -> () {
    let files = generate_docker_files(config_file);
    for file in files.iter() {
        write_file(&mut file.1.clone(), String::from("./") + &config_file.project_name + "/" + &file.0.clone());
    }
}

fn generate_auth(config_file: &ConfigFile) -> () {
    let auth_files = generate_auth_files(config_file);
    for file in auth_files.iter() {
        write_file(&mut file.1.clone(), String::from("./") + &config_file.project_name + "/" + &file.0.clone());
    }
}

fn generate_routes(config_file: &ConfigFile, models_file: &ModelFile) -> () {
    let routes = generate_routes_files(config_file, models_file);
    for model in routes.iter() {
        write_file(&mut model.1.clone(), String::from("./") + &config_file.project_name + "/app/src/routes/" + &model.0);
    }
}

fn generate_dotenv(config_file: &ConfigFile) -> () {
    let mut dotenv = generate_dotenv_file(config_file, false);
    write_file(&mut dotenv, String::from("./") + &config_file.project_name + "/app/.env");
    let mut dotenv_sample = generate_dotenv_sample_file(config_file, false);
    write_file(&mut dotenv_sample, String::from("./") + &config_file.project_name + "/app/.env.sample");
}

fn generate_models(config_file: &ConfigFile, models_file: &ModelFile) -> () {
    let core_models = generate_core_models(config_file);
    for model in core_models.iter() {
        write_file(&mut model.1.clone(), String::from("./") + &config_file.project_name + "/app/src/models/core/" + &model.0);
    }
    let app_models = generate_app_models(config_file, models_file);
    for model in app_models.iter() {
        write_file(&mut model.1.clone(), String::from("./") + &config_file.project_name + "/app/src/models/" + &model.0);
    }
}

fn generate_server(config_file: &ConfigFile, models_file: &ModelFile) -> () {
    let mut code = generate_server_file(config_file, models_file);
    write_file(&mut code, String::from("./") + &config_file.project_name + "/app/src/server.ts");
}
fn generate_table_config(config_file: &ConfigFile, models: &ModelFile) -> () {
    match config_file.database {
        DbType::SQLITE => {
            let db_config = DbTableStructure::new(config_file, models);
            let mut serialized = serde_json::to_string_pretty(&db_config).unwrap();
            write_file(
                &mut serialized,
                String::from("./") + &config_file.project_name + "/app/src/db/database.config.json",
            );
        }
    }
}

fn generate_orm(config_file: &ConfigFile) -> () {
    match config_file.database {
        DbType::SQLITE => {
            write_file(&mut generate_orm_code(), String::from("./") + &config_file.project_name + "/app/src/db/database.handler.ts");
        }
    }
    
}

fn generate_ts_config(config_file: &ConfigFile) -> () {
    let ts_config = TsCompilerOptions::new();
    let mut serialized = serde_json::to_string_pretty(&ts_config).unwrap();
    write_file(&mut serialized, String::from("./") + &config_file.project_name + "/app/tsconfig.json");
}

fn generate_migration(config_file: &ConfigFile, models: &ModelFile) -> () {
    match config_file.database {
        DbType::SQLITE => {
            write_file(&mut generate_sqlite_migation_files(config_file, models), String::from("./") + &config_file.project_name + "/app/migrations/001-inital-schema.sql");
        }
    }
}

fn generate_node_package_json(config_file: &ConfigFile) -> () {
    let package_json = NodePackage::new(config_file);
    let mut serialized = serde_json::to_string_pretty(&package_json).unwrap();
    write_file(&mut serialized, String::from("./") + &config_file.project_name + "/app/package.json");
}

fn generate_folder_structure(config_file: &ConfigFile) -> () {
    println!("Generating folders...");
    generate_folder(String::from("./") + &config_file.project_name);
    generate_folder(String::from("./") + &config_file.project_name + "/app");
    generate_folder(String::from("./") + &config_file.project_name + "/app/migrations");
    generate_folder(String::from("./") + &config_file.project_name + "/app/src/models");
    generate_folder(String::from("./") + &config_file.project_name + "/app/src/models/core");
    generate_folder(String::from("./") + &config_file.project_name + "/app/src/routes");
    generate_folder(String::from("./") + &config_file.project_name + "/app/src/db");
    if config_file.auth {
        generate_folder(String::from("./") + &config_file.project_name + "/app/src/auth");
    }
    match config_file.database {
        DbType::SQLITE => generate_folder(String::from("./") + &config_file.project_name + "/app/db")
    }
    println!("Done generating folders");
}
