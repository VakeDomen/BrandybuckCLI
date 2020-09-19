use crate::db_generators::db_types::DbType;
use crate::helpers::util_helper::random_key;

#[derive(Serialize, Deserialize)]
#[serde(default = "default_config")]
pub struct ConfigFile {
    pub project_name: String,
    pub port: i32,
    pub auth: bool,
    pub model_source: String,
    pub database: DbType,
    pub documentation: bool,
    pub docker: Docker,
    pub log: bool
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(default = "default_traefik")]
pub struct TraefikConfig {
    pub container_name: String,
    pub proxy_network_name: String,
    pub certresolver_name: String,
    pub entrypoint_name: String,
    pub domain: String
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(default = "default_docker")]
pub struct DockerConfig {
    pub port: i32,
    pub traefik2: Traefik,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Docker {
    Bool(bool),
    Config(DockerConfig)
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Traefik {
    Bool(bool),
    Config(TraefikConfig)
}

fn default_config() -> ConfigFile {
    ConfigFile {
        project_name: String::from("Unknown"),
        port: 3000,
        auth: false,
        model_source: String::from("./brandybuck.models.json"),
        database: DbType::SQLITE,
        documentation: true,
        docker: Docker::Config(default_docker()),
        log: true
    }
}

fn default_docker() -> DockerConfig {
    DockerConfig {
        port: 3000,
        traefik2: Traefik::Bool(false)
    }
}

fn default_traefik() -> TraefikConfig {
    TraefikConfig {
        container_name: String::from("brandybuck_traefik_container_") + &random_key(5),
        proxy_network_name: String::from("proxy"),
        certresolver_name: String::from("le"),
        entrypoint_name: String::from("websecure"),
        domain: String::from("www.projectname.com")
    }
}

impl TraefikConfig {
    pub fn new() -> TraefikConfig {
        default_traefik()
    }
}

impl DockerConfig {
    pub fn new() -> DockerConfig {
        default_docker()
    }
}


impl ConfigFile {
    pub fn new() -> ConfigFile {
        default_config()
    }

    pub fn get_traefik2_conf(&self) -> Option<TraefikConfig> {
        match self.docker.clone() {
            Docker::Bool(b) => {
                if b { 
                    match DockerConfig::new().traefik2 {
                        Traefik::Bool(b) => {
                            if b {
                                Some(TraefikConfig::new())
                            } else {
                                None
                            }
                        }
                        Traefik::Config(conf) => {
                            Some(conf)
                        }
                    }
                } else {
                    None
                }
            },
            Docker::Config(doc_conf) => {
                match doc_conf.traefik2 {
                    Traefik::Bool(b) => {
                        if b {
                            Some(TraefikConfig::new())
                        } else {
                            None
                        }
                    }
                    Traefik::Config(conf) => {
                        Some(conf.clone())
                    }
                }
            }
        }
    }
}