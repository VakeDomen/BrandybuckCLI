use serde_json::Value;
use crate::models::config_file::ConfigFile;
#[derive(Serialize, Deserialize)]
pub struct NodePackage {
    name: String,
    version: String,
    description: String,
    main: String,
    scripts: Value,
    repository: Value,
    author: String,
    license: String,
    bugs: Bugs,
    homepage: String,
    dependencies: Value,
}

#[derive(Serialize, Deserialize)]
struct Bugs {
    url: String
}


// impl NodePackage {
//     pub fn new(&config: ConfigFile) -> NodePackage {
//         NodePackage {
//             name: config.name.clone(),
//             version: String::from("1.0.0"),
//             description: Sting::from("Backend application for" + config.name.clone()),
//             main: String::from("src/server.ts");
//             scripts:
//         }
//     }
// }


// struct Dependency {
//     @types/express:  String,
//     @types/node:  String,
//     @types/request:  String,
//     bcrypt: String,
//     body-parser:  String,
//     cors: String,
//     dotenv: String,
//     express:  String,
//     jsonwebtoken: String,
//     morgan: String,
//     request:  String,
//     sqlite: String,
//     sqlite3: String,
//     ts-node-dev: String,
//     typescript: String,
//     url:  String,
//     uuid: String
// }