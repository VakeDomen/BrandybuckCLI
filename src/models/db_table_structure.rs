use crate::models::brandybuck_models_file::ModelFile;
use crate::models::brandybuck_config_file::ConfigFile;
use serde_json::{Value, Map};

#[derive(Serialize, Deserialize)]
pub struct DbTableStructure {
    tables: Map<String, Value>,
    log: bool
}


impl DbTableStructure {
    pub fn new(config: &ConfigFile, models: &ModelFile) -> DbTableStructure {
        let mut db_structure = DbTableStructure {
           tables: generate_tables(models),
           log: config.log.clone()
        };
        if config.auth {
            db_structure.tables.insert(String::from("user"), Value::String(String::from("users")));
        }
        db_structure
    }
}

fn generate_tables(models: &ModelFile) -> Map<String, Value> {
    let mut map = Map::new();
    for model in models.models.iter() {
        map.insert(String::from(model.name.clone()), Value::String(String::from(model.name.clone() + "s")));
    }
    map
}