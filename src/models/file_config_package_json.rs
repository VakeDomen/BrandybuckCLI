use serde_json::{Value, Map};
use crate::models::brandybuck_config_file::ConfigFile;
#[derive(Serialize, Deserialize)]
pub struct NodePackage {
    name: String,
    version: String,
    description: String,
    main: String,
    scripts: Map<String, Value>,
    repository: Map<String, Value>,
    author: String,
    license: String,
    bugs: Map<String, Value>,
    homepage: String,
    dependencies: Map<String, Value>,
}

impl NodePackage {
    pub fn new(config: &ConfigFile) -> NodePackage {
        NodePackage {
            name: config.project_name.clone(),
            version: String::from("1.0.0"),
            description: String::from("Backend application for") + &config.project_name.clone(),
            main: String::from("src/server.ts"),
            scripts: generate_scripts(config),
            repository: generate_repository(config),
            author: String::from("Author"),
            license: String::from("ISC"),
            bugs: generate_bugs(config),
            homepage: String::from("GitHub link here"),
            dependencies: generate_dependencies(config)
        }
    }
}
fn generate_bugs(config: &ConfigFile) -> Map<String, Value> {
    let mut map = Map::new();
    map.insert(String::from("url"), Value::String(String::from("git+https://github.com/_/issues")));
    map
}

fn generate_scripts(config: &ConfigFile) -> Map<String, Value> {
    let mut map = Map::new();
    map.insert(String::from("start"), Value::String(String::from("ts-node-dev --respawn --transpileOnly ./src/server.ts")));
    map.insert(String::from("prod"), Value::String(String::from("tsc && node ./build/server.js")));
    map.insert(String::from("tsc"), Value::String(String::from("tsc")));
    map    
}

fn generate_repository(config: &ConfigFile) -> Map<String, Value> {
    let mut map = Map::new();
    map.insert(String::from("type"), Value::String(String::from("git")));
    map.insert(String::from("url"), Value::String(String::from("git+https://github.com/")));
    map
}

fn generate_dependencies(config: &ConfigFile) -> Map<String, Value> {
    let mut map = Map::new();
    // TS
    map.insert(String::from("typescript"), Value::String(String::from("^3.7.5")));
    map.insert(String::from("ts-node-dev"), Value::String(String::from("^1.0.0-pre.44")));
    map.insert(String::from("@types/node"), Value::String(String::from("^13.7.4")));
    // types
    map.insert(String::from("@types/express"), Value::String(String::from("^4.17.2")));
    map.insert(String::from("@types/node"), Value::String(String::from("^13.7.4")));
    map.insert(String::from("@types/node"), Value::String(String::from("^13.7.4")));
    // web
    map.insert(String::from("body-parser"), Value::String(String::from("^1.19.0")));
    map.insert(String::from("cors"), Value::String(String::from("^2.8.5")));
    map.insert(String::from("express"), Value::String(String::from("^4.17.1")));
    map.insert(String::from("url"), Value::String(String::from("^0.11.0")));
    // utils
    map.insert(String::from("uuid"), Value::String(String::from("^3.4.0")));
    map.insert(String::from("morgan"), Value::String(String::from("^1.9.1")));
    map.insert(String::from("dotenv"), Value::String(String::from("^8.2.0")));
    // auth
    if config.auth {
        map.insert(String::from("bcrypt"), Value::String(String::from("^5.0.0")));
        map.insert(String::from("jsonwebtoken"), Value::String(String::from("^8.5.1")));
    }
    if config.database == String::from("sqlite") {
        map.insert(String::from("sqlite"), Value::String(String::from("^3.0.3")));
        map.insert(String::from("sqlite3"), Value::String(String::from("^4.1.1")));
    }
    map
}