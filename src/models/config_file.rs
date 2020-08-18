use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct ConfigFile {
    auth: bool,
    model_source: PathBuf,
    database: String
}

impl ConfigFile {
    pub fn new() -> ConfigFile {
        ConfigFile{
            auth: true,
            model_source: PathBuf::from("./models.brandybuck.json"),
            database: String::from("sqlite")
        }
    }
}