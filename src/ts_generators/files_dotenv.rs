use crate::models::brandybuck_config_file::ConfigFile;
use crate::db_generators::db_types::DbType;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

pub fn generate_dotenv_file(config_file: &ConfigFile) -> String {
    let mut code = Vec::new();
    code.push(String::from("PORT=") + &config_file.port.to_string());
    if config_file.auth {
        code.push(String::from("JWT_SECRET=") + &random_key());
        code.push(String::from("ADMIN_EMAIL=your@email.com"));
    }
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
    if config_file.auth {
        code.push(String::from("JWT_SECTRET="));
        code.push(String::from("ADMIN_EMAIL="));
    }
    match config_file.database {
        DbType::SQLITE => {
            code.push(String::from("SQLITE_DB="))
        }
    }
    code.join("\n")
}

fn random_key() -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .collect()
}