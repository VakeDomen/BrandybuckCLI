use crate::models::brandybuck_config_file::ConfigFile;
use crate::models::brandybuck_models_file::ModelFile;

pub fn generate_core_models(
    config_file: &ConfigFile,
    models_file: &ModelFile,
) -> Vec<(String, String)> {
    let mut files: Vec<(String, String)> = Vec::new();
    files.push((
        String::from("db.item.ts"),
        generate_db_item_model(config_file),
    ));
    files.push((
        String::from("response.ts"),
        generate_response_model(config_file),
    ));
    files.push((
        String::from("error.response.ts"),
        generate_error_response_model(config_file),
    ));
    files.push((
        String::from("success.response.ts"),
        generate_success_response_model(config_file),
    ));
    files
}

pub fn generate_user_models(
    config_file: &ConfigFile,
    models_file: &ModelFile,
) -> Vec<(String, String)> {
    let mut files: Vec<(String, String)> = Vec::new();
    files.push((String::from(""), String::from("")));
    files.push((String::from(""), String::from("")));
    files.push((String::from(""), String::from("")));
    files.push((String::from(""), String::from("")));
    files
}

fn generate_db_item_model(config_file: &ConfigFile) -> String {
    let mut code = Vec::new();
    code.push(String::from("const uuidv4 = require('uuid/v4');\n"));
    code.push(String::from("export class DbItem {\n"));
    code.push(String::from("\tid: string | undefined;\n"));
    code.push(String::from("\tconstructor(data: any | undefined){\n\t\tif (typeof data !== 'undefined') {\n\t\t\tthis.id = data.id;\n\t\t}\n\t}"));
    code.push(String::from(
        "\tgenerateId(): void {\n\t\tthis.id = uuidv4();\n\t}",
    ));
    code.push(String::from("\tisEmpty(): boolean {\n\t\tfor (let data in this){\n\t\t\tif (typeof this[data] !== 'undefined'){\n\t\t\t\treturn false;\n\t\t\t}\n\t\t}\n\t\treturn true;\n\t}"));
    code.push(String::from("\tvaluesToString(): string {\n\t\tconst str: string[] = [];\n\t\tfor (let key of Object.keys(this)){\n\t\t\tif (typeof this[key] !== 'undefined'){\n\t\t\t\tif (key !== 'id') {\n\t\t\t\t\tif (typeof this[key] === 'boolean') {\n\t\t\t\t\t\tstr.push(key + ' = ' + ((this[key])? 1 : 0));\n\t\t\t\t\t} else {\n\t\t\t\t\t\tstr.push(key + ' = `' + this[key] + '`');\n\t\t\t\t\t}\n\t\t\t\t}\n\t\t\t}\n\t\t}\n\t\treturn str.join(', ');\n\t}"));
    code.push(String::from("\twhereSimilarString(): string {\n\t\tconst str: string[] = [];\n\t\tfor (let key of Object.keys(this)) {\n\t\t\tif (typeof this[key] !== 'undefined') {\n\t\t\t\tif (key === 'id') continue;\n\t\t\t\tif (typeof this[key] === 'boolean') {\n\t\t\t\t\tstr.push(key + ' = ' + ((this[key] ? 1 : 0)));\n\t\t\t\t\tcontinue;\n\t\t\t\t}\n\t\t\t\tstr.push('UPPER(' + key + ') LIKE UPPER(`%' + this[key] + '%`)');\n\t\t\t}\n\t\t}\n\t\treturn str.join(', ');\n\t}"));
    code.push(String::from("\tlistKeys(): string{\n\t\tconst str: string[] = [];\n\t\tfor (let data of Object.keys(this)){\n\t\t\tif (typeof this[data]!= 'undefined'){\n\t\t\t\tstr.push(data);\n\t\t\t}\n\t\t}\n\t\treturn str.join(', ');\n\t}"));
    code.push(String::from("\twhereString(): string {\n\t\tconst str: string[] = [];\n\t\tfor (let data of Object.keys(this)){\n\t\t\tif (typeof this[data] != 'undefined') {\n\t\t\t\tif (typeof this[data] === 'number') {\n\t\t\t\t\tstr.push(data + ' = `' + this[data] + '`');\n\t\t\t\t} else if (typeof this[data] === 'boolean') {\n\t\t\t\t\tstr.push(data + ' = `' + ((this[data])? 1 : 0) + '`');\n\t\t\t\t} else {\n\t\t\t\t\tstr.push(data + ' = `' + this[data] + '`');\n\t\t\t\t}\n\t\t\t}\n\t\t}\n\t\tif (str === []) return '1';\n\t\treturn str.join(' AND ');\n\t}"));
    code.push(String::from("\tlistValues(): string{\n\t\tconst str: string[] = [];\n\t\tfor (let data of Object.keys(this)) {\n\t\t\tif (typeof this[data]!= 'undefined') {\n\t\t\t\tif (typeof this[data] === 'boolean') {\n\t\t\t\t\tstr.push('`' + ((this[data])? 1 : 0) + '`');\n\t\t\t\t} else {\n\t\t\t\t\tstr.push('`' + this[data] + '`');\n\t\t\t\t}\n\t\t\t}\n\t\t}\n\t\treturn str.join(', ');\n\t}"));
    code.push(String::from("\tupdateValues(updates: DbItem): void {\n\tfor (let key of Object.keys(updates)) {\n\t\t\tif (key !== 'id') this[key] = updates[key];\n\t\t}\n\t}"));
    code.push(String::from("}"));
    code.join("\n")
}

fn generate_response_model(config_file: &ConfigFile) -> String {
    let mut code: Vec<String> = Vec::new();

    code.join("\n")
}

fn generate_error_response_model(config_file: &ConfigFile) -> String {
    let mut code: Vec<String> = Vec::new();

    code.join("\n")
}

fn generate_success_response_model(config_file: &ConfigFile) -> String {
    let mut code: Vec<String> = Vec::new();

    code.join("\n")
}
