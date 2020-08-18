use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct ConfigFile {
    project_name: String,
    auth: bool,
    model_source: PathBuf,
    database: String,
    documentation: bool
}

impl ConfigFile {
    pub fn new() -> ConfigFile {
        ConfigFile {
            project_name: String::from("test_project"),
            auth: true,
            model_source: PathBuf::from("./models.brandybuck.json"),
            database: String::from("sqlite"),
            documentation: true
        }
    }
}