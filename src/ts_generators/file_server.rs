use crate::models::brandybuck_config_file::ConfigFile;
use crate::models::brandybuck_models_file::ModelFile;

pub fn generate_server_file(config_file: &ConfigFile, models_file: &ModelFile) -> String {
    let mut code = Vec::new();
    code.push(generate_imports(config_file));
    code.push(generate_dotenv_checks(config_file));
    code.push(generate_initialisation(config_file));
    code.push(generate_route_binding(config_file));
    code.join("\n")
}

fn generate_route_binding(config_file: &ConfigFile) -> String {
    let mut code = Vec::new();
    code.push(String::from("const absPath = path.resolve(__dirname, '.');"));
    code.push(String::from("fs.readdir(absPath + '/routes/', (err: Error, files: string[]) => {"));
    code.push(String::from("\tif (err) {\n\t\tconsole.log('Error processing routes!', err);\n\t\tprocess.exit(1);\n\t}"));
    code.push(String::from("\tfiles.forEach((routeFileName: string) => {\n\t\tconsole.log('Importing ' + routeFileName + '...');\n\t\tapp.use(require(absPath + '/routes/' + routeFileName));\n\t});"));
    code.push(String::from("\tapp.use((req: express.Request, res: express.Response, next: any) => {\n\t\tconst error = new Error('Not found');\n\t\tnext(error);\n\t});"));
    code.push(String::from("\tapp.use((error: any, req: express.Request, res: express.Response, next: any) => {\n\t\tres.status(error.status || 500);\n\t\tres.json({message: error.message});\n\t});"));
    code.push(String::from("console.log('Backend sucessfully initialised!');"));
    code.push(String::from("});"));
    code.join("\n")
}

fn generate_database_init(config_file: &ConfigFile) -> String {
    let mut code = Vec::new();
    code.push("const db = Promise.resolve().then(() => sqlite.open(dbPath)).then(db => {\n\t// db.migrate({ force: 'last' });\n\tdb.migrate({});\n});");
    code.join("\n")
}

fn generate_dotenv_checks(config_file: &ConfigFile) -> String {
    let mut code = Vec::new();
    code.push(String::from("if (!process.env.PORT) {\n\tconsole.log('Port not specified!');\n\tprocess.exit(1);\n}"));
    if config_file.database == String::from("sqlite") {
        code.push(String::from("const dbPath: string = process.env.SQLITE_DB || './src/db/data/sqlite.db';"));
    }
    code.join("\n")
}

fn generate_middleware(config_file: &ConfigFile) -> String {
    let mut code = Vec::new();
    code.push(String::from("app.use(morgan('dev'));"));
    code.push(String::from("app.use(bodyParser.json({limit: '50mb'}));\napp.use(bodyParser.urlencoded({extended: true, limit: '50mb'}));"));
    code.join("\n")
}

fn generate_initialisation(config_file: &ConfigFile) -> String {
    let mut code = Vec::new();
    code.push(String::from("console.log('Initialising backend...');"));
    code.push(generate_database_init(config_file));
    code.push(String::from("const app: express.Application = express();"));
    code.push(generate_middleware(config_file));
    code.join("\n")
}

fn generate_imports(config_file: &ConfigFile) -> String {
    let mut code = Vec::new();
    if config_file.log {
        code.push(String::from("console.log('Importing dependencies...')"));
    }
    code.push(String::from("require('dotenv').config();"));
    code.push(String::from("import express = require('express');"));

    code.push(import_database(config_file));
    code.push(String::from("const cors = require('cors');"));
    code.push(String::from("const morgan = require('morgan');"));
    code.push(String::from("const bodyParser = require('body-parser');"));
    code.push(String::from("const fs = require('fs');"));
    code.push(String::from("const path = require('path');"));
    if config_file.log {
        code.push(String::from("console.log('Finished importing!')"));
    }
    code.join("\n")
}

fn import_database(config_file: &ConfigFile) -> String {
    let mut code = Vec::new();
    if config_file.database == String::from("sqlite") {
        code.push(String::from("import sqlite from 'sqlite';"));
    }
    code.join("\n")
}