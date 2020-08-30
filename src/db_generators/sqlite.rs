use crate::models::brandybuck_config_file::ConfigFile;
use crate::models::brandybuck_models_file::{ModelFile, Model};

pub fn generate_sqlite_migation_files(config_file: &ConfigFile, model_file: &ModelFile) -> String {
    let table_models_up = generate_tables_up(model_file);
    let table_models_down = generate_tables_down(model_file);
    let migration_string = String::from("-- Up\n") + &table_models_up + "\n-- Down\n" + &table_models_down;
    migration_string
}

pub fn generate_orm_code() -> String {
    let mut code = Vec::new();
    code.push(generate_imports());
    code.push(generate_connection());
    code.push(generate_query_executor());
    code.push(generate_queries());
    code.join("\n")
}

fn generate_queries() -> String {
    let mut queries = Vec::new();
    queries.push(fetch_query());
    queries.push(fetch_similar());
    queries.push(fetch_all());
    queries.push(insert());
    queries.push(update());
    queries.push(delete());
    queries.push(inner_join());
    queries.push(left_join());
    queries.join("\n")
}

fn generate_tables_up(model_file: &ModelFile) -> String {
    let mut tables: Vec<String> = Vec::new();
    for model in model_file.models.iter() {
        tables.push(generate_model_table(model));
    }
    tables.join("\n")
}

fn generate_tables_down(model_file: &ModelFile) -> String {
    let mut tables: Vec<String> = Vec::new();
    for model in model_file.models.iter() {
        tables.push(String::from("DROP TABLE ") + &model.name + ";");
    }
    tables.join("\n")
}

fn generate_model_table(model: &Model) -> String {
    let mut table_rows = Vec::new();
    table_rows.push(String::from("CREATE TABLE ") + &model.name + "s (");
    table_rows.push(String::from("\tid VARCHAR PRIMARY KEY,"));
    for col in model.fields.iter() {
        let mut column = String::from("\t") + &col.name + " " + &col.data_type + " ";
        if col.null {
            column = column + "NULL,";
        } else {
            column = column + "NOT NULL,";
        }
        table_rows.push(column)
    }
    let row_len = &table_rows.len() - 1;
    let str_len = &table_rows[row_len.clone()].len() - 1;
    table_rows[row_len].remove(str_len);
    table_rows.push(String::from(");\n"));
    table_rows.join("\n")
}

fn generate_imports() -> String {
    "const sqlite = require(\"sqlite\");\nimport { DbItem } from '../models/core/db.item';\nimport * as config from './database.config.json';\n".to_string()
}

fn generate_connection() -> String {
    "const dbConnection: Promise<any> = sqlite.open(process.env.SQLITE_DB || './src/db/data/sqlite.db', { Promise });\n".to_string()
}

fn generate_query_executor() -> String {
    "export async function query<T>(query: string): Promise<T[]> {\n\tconst db = await dbConnection;\n\tif (config.log) {\n\t\tconsole.log(query);\n\t}\n\treturn db.all(query);\n}".to_string()
}

fn fetch_query() -> String {
    "export async function fetch<T>(table: string, filter: DbItem): Promise<T[]> {\n\treturn query<T>('SELECT * FROM ' + table + ' WHERE ' + filter.whereString() + ';');\n}".to_string()
}

fn fetch_similar() -> String {
    "export async function fetchSimilar<T>(table: string, filter: DbItem): Promise<T[]> {\n\treturn query<T>('SELECT * FROM ' + table + ' WHERE ' + filter.whereSimilarString() + ';');\n}".to_string()
}

fn fetch_all() -> String {
    "export async function fetchAll<T>(table: string): Promise<T[]> {\n\treturn query<T>('SELECT * FROM ' + table + ';');\n}".to_string()
}

fn insert() -> String {
    "export async function insert<T>(table: string, filter: DbItem): Promise<T[]> {\n\treturn query<T>('INSERT INTO ' + table + ' (' + filter.listKeys() + ') VALUES (' + filter.listValues() + ');');\n}".to_string()
}

fn update() -> String {
    "export async function update<T>(table: string, filter: DbItem): Promise<T[]> {\n\treturn query<T>('UPDATE ' + table + ' SET ' + filter.valuesToString() + ' WHERE id=\\'' + filter.id + '\\';');\n}".to_string()
}

fn delete() -> String {
    "export async function deleteItem<T>(table: string, filter: DbItem): Promise<T[]> {\n\treturn query<T>('DELETE FROM ' + table + ' WHERE ' + filter.whereString() + ';');\n}".to_string()
}

fn inner_join() -> String {
    "export async function innerJoin<T>(t1: string, t2: string, t1key: string, t2key: string, filter: DbItem): Promise<T[]> {\n\treturn query<T>('SELECT * FROM ' + t1 + ' AS t1 INNER JOIN ' + t2 + ' as t2 ON t1.' + t1key + ' = t2.' + t2key + ' WHERE ' + filter.whereString() + ';');\n}".to_string()
}

fn left_join() -> String {
    "export async function leftJoin<T>(t1: string, t2: string, t1key: string, t2key: string, filter: DbItem): Promise<T[]> {\n\treturn query<T>('SELECT * FROM ' + t1 + ' AS t1 LEFT JOIN ' + t2 + ' as t2 ON t1.' + t1key + ' = t2.' + t2key + ' WHERE ' + filter.whereString() + ';');\n}".to_string()
}
