use std::path::PathBuf;
use crate::db_generators::db_types::DbType;

#[derive(Serialize, Deserialize)]
pub struct ConfigFile {
    pub project_name: String,
    pub port: i32,
    pub auth: bool,
    pub model_source: String,
    pub database: DbType,
    pub documentation: bool,
    pub docker: bool,
    pub log: bool
}

impl ConfigFile {
    pub fn new() -> ConfigFile {
        ConfigFile {
            project_name: String::from("test_project"),
            port: 3000,
            auth: false,
            model_source: String::from("./brandybuck.models.json"),
            database: DbType::SQLITE,
            documentation: true,
            docker: true,
            log: true
        }
    }
}