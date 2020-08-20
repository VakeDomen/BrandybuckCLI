use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct ConfigFile {
    pub project_name: String,
    pub auth: bool,
    pub model_source: String,
    pub database: String,
    pub documentation: bool,
    pub docker: bool,
    pub log: bool
}

impl ConfigFile {
    pub fn new() -> ConfigFile {
        ConfigFile {
            project_name: String::from("test_project"),
            auth: true,
            model_source: String::from("./brandybuck.models.json"),
            database: String::from("sqlite"),
            documentation: true,
            docker: true,
            log: true
        }
    }
}