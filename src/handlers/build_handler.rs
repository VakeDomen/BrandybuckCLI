use crate::db_generators::sqlite::{generate_orm_code, generate_sqlite_migation_files};
use crate::helpers::file_helper::{generate_folder, read_config_file, read_model_file, write_file};
use crate::models::brandybuck_config_file::ConfigFile;
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

pub fn build_application() -> () {
    let config_file: ConfigFile = read_config_file();
    let models_file: ModelFile = read_model_file(config_file.model_source.clone());
    generate_folder_structure(&config_file);
    generate_node_package_json(&config_file);
    generate_ts_config();
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
}

fn generate_auth(config_file: &ConfigFile) -> () {
    let auth_files = generate_auth_files(config_file);
    for file in auth_files.iter() {
        write_file(&mut file.1.clone(), file.0.clone());
    }
}

fn generate_routes(config_file: &ConfigFile, models_file: &ModelFile) -> () {
    let routes = generate_routes_files(config_file, models_file);
    for model in routes.iter() {
        write_file(&mut model.1.clone(), "app/src/routes/".to_string() + &model.0);
    }
}

fn generate_dotenv(config_file: &ConfigFile) -> () {
    let mut dotenv = generate_dotenv_file(config_file);
    write_file(&mut dotenv, "app/.env".to_string());
    let mut dotenv_sample = generate_dotenv_sample_file(config_file);
    write_file(&mut dotenv_sample, "app/.env.sample".to_string());
}

fn generate_models(config_file: &ConfigFile, models_file: &ModelFile) -> () {
    let core_models = generate_core_models(config_file);
    for model in core_models.iter() {
        write_file(&mut model.1.clone(), "app/src/models/core/".to_string() + &model.0);
    }
    let app_models = generate_app_models(config_file, models_file);
    for model in app_models.iter() {
        write_file(&mut model.1.clone(), "app/src/models/".to_string() + &model.0);
    }
}

fn generate_server(config_file: &ConfigFile, models_file: &ModelFile) -> () {
    let mut code = generate_server_file(config_file, models_file);
    write_file(&mut code, "app/src/server.ts".to_string());
}
fn generate_table_config(config: &ConfigFile, models: &ModelFile) -> () {
    match config.database {
        DbType::SQLITE => {
            let db_config = DbTableStructure::new(config, models);
            let mut serialized = serde_json::to_string_pretty(&db_config).unwrap();
            write_file(
                &mut serialized,
                "app/src/db/database.config.json".to_string(),
            );
        }
    }
}

fn generate_orm(config: &ConfigFile) -> () {
    let mut orm_code = "".to_string();
    match config.database {
        DbType::SQLITE => {
            orm_code = generate_orm_code();
        }
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
    match config.database {
        DbType::SQLITE => {
            migration_file = generate_sqlite_migation_files(config, models);
            file_name = "app/migrations/001-inital-schema.sql".to_string();
        }
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
    match config.database {
        DbType::SQLITE => generate_folder(String::from("./app/db"))
    }
    println!("Done generating folders");
}
