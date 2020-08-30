use crate::models::brandybuck_config_file::ConfigFile;
use crate::db_generators::db_types::DbType;

pub fn generate_dotenv_file(config_file: &ConfigFile) -> String {
    let mut code = Vec::new();
    code.push(String::from("PORT=") + &config_file.port.to_string());
    match config_file.database {
        DbType::SQLITE => {
            code.push(String::from("SQLITE_DB=./db/sqlite.db"))
        }
    }
    code.join("\n")
}

pub fn generate_dotenv_sample_file(config_file: &ConfigFile) -> String {
    let mut code = Vec::new();
    code.push(String::from("PORT="));
    match config_file.database {
        DbType::SQLITE => {
            code.push(String::from("SQLITE_DB="))
        }
    }
    code.join("\n")
}