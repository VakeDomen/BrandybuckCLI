use crate::models::brandybuck_config_file::{ConfigFile, Docker, Traefik, TraefikConfig};
use crate::db_generators::db_types::DbType;
use crate::helpers::util_helper::random_key;

pub fn generate_dotenv_file(config_file: &ConfigFile, is_docker: bool) -> String {
    let mut code = Vec::new();
    code.push(String::from("PORT=") + &config_file.port.to_string());
    if config_file.auth {
        code.push(String::from("JWT_SECRET=") + &random_key(30));
        code.push(String::from("ADMIN_EMAIL=your@email.com"));
        code.push(String::from("HASH_SALT_ROUNDS=10"));
    }
    match config_file.database {
        DbType::SQLITE => {
            code.push(String::from("SQLITE_DB=./db/sqlite.db"))
        }
    }
    if is_docker {
        match &config_file.docker {
            Docker::Bool(_) => (),
            Docker::Config(docker_conf) => {
                match &docker_conf.traefik2 {
                    Traefik::Bool(b) => if *b { code.push(String::from("DOMAIN=") + &TraefikConfig::new().domain) },
                    Traefik::Config(traefik_config) => code.push(String::from("DOMAIN=") + &traefik_config.domain)
                }
            }
        }
    }
    code.join("\n")
}

pub fn generate_dotenv_sample_file(config_file: &ConfigFile, is_docker: bool) -> String {
    let mut code = Vec::new();
    code.push(String::from("PORT="));
    if config_file.auth {
        code.push(String::from("JWT_SECTRET="));
        code.push(String::from("ADMIN_EMAIL="));
        code.push(String::from("HASH_SALT_ROUNDS="));
    }
    match config_file.database {
        DbType::SQLITE => {
            code.push(String::from("SQLITE_DB="))
        }
    }
    if is_docker {
        match &config_file.docker {
            Docker::Bool(_) => (),
            Docker::Config(docker_conf) => {
                match &docker_conf.traefik2 {
                    Traefik::Bool(b) => if *b { code.push(String::from("DOMAIN=")) },
                    Traefik::Config(_) => code.push(String::from("DOMAIN="))
                }
            }
        }
    }
    
    code.join("\n")
}

pub fn generate_docker_environment(config_file: &ConfigFile) -> String {
    let mut code = Vec::new();
    code.push(String::from("      - PORT=${PORT}"));
    if config_file.auth {
        code.push(String::from("      - JWT_SECTRET=${JWT_SECRET}"));
        code.push(String::from("      - ADMIN_EMAIL=${ADMIN_EMAIL}"));
        code.push(String::from("      - HASH_SALT_ROUNDS=${HASH_SALT_ROUNDS}"))
    }
    match config_file.database {
        DbType::SQLITE => {
            code.push(String::from("      - SQLITE_DB=${SQLITE_DB}"))
        }
    }
    code.join("\n")
}

