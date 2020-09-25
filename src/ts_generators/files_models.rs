use crate::models::brandybuck_config_file::ConfigFile;
use crate::models::brandybuck_models_file::{ModelFile, Model};
use crate::helpers::util_helper::first_letter_to_uppper_case;
use crate::db_generators::db_types::map_field_to_ts_type;


pub fn generate_app_models(
    config_file: &ConfigFile,
    models_file: &ModelFile,
) -> Vec<(String, String)> {
    let mut files: Vec<(String, String)> = Vec::new();
    for model in models_file.models.iter() {
        let model_item_file = generate_app_model(model, config_file);
        files.push((
            model.name.clone() + ".item.ts",
            model_item_file
        ));
    }
    if config_file.auth {
        files.push((
            String::from("user.item.ts"), 
            generate_user_model()
        ));
    }
    files
}

pub fn generate_core_models(
    config_file: &ConfigFile,
) -> Vec<(String, String)> {
    let mut files: Vec<(String, String)> = Vec::new();
    files.push((
        String::from("db.item.ts"),
        generate_db_item_model(),
    ));
    files.push((
        String::from("response.ts"),
        generate_response_model(),
    ));
    files.push((
        String::from("error.response.ts"),
        generate_error_response_model(),
    ));
    files.push((
        String::from("success.response.ts"),
        generate_success_response_model(),
    ));
    if config_file.auth {
        files.push((
            String::from("local.credentials.ts"),
            generate_local_credentials()
        ))
    }
    files
}

pub fn generate_local_credentials() -> String {
    let mut code = Vec::new();
    code.push(String::from("export class LocalCredentials {"));
    code.push(String::from("\temail: string;\n\tpassword: string;"));
    code.push(String::from("\tconstructor(email: string, password: string) {\n\t\tthis.email = email;\n\t\tthis.password = password;\n\t}"));
    code.push(String::from("}"));
    code.join("\n")
}

pub fn generate_user_model() -> String {
    let mut code = Vec::new();
    code.push(String::from("import { DbItem } from './core/db.item';\nimport * as bcrypt from 'bcrypt';"));
    code.push(String::from("export class User extends DbItem {"));
    code.push(String::from("\tname: string;email: string;\tpassword: string;\trole: 'USER' | 'ADMIN' = 'USER';"));
    code.push(String::from("\tconstructor(data: any) {\n\t\tsuper(data);\n\t\tthis.name = data.name;\n\t\tthis.email = data.email;\n\t\tthis.password = data.password;\n\t\tif (process.env.ADMIN_EMAIL === data.email) {\n\t\t\tthis.role = 'ADMIN';\n\t\t}\n\t}"));
    code.push(String::from("\tisValid(): boolean {\n\t\tif (!(this.name && this.email && this.password && this.role)) {\n\t\t\treturn false;\n\t\t}\n\t\tif (this.role === 'ADMIN' && this.email !== process.env.ADMIN_EMAIL) {\n\t\t\treturn false;\n\t\t}\n\t\treturn true;\n\t}"));
    code.push(String::from("\tasync hashPassword(): Promise<void> {\n\t\tthis.password = await bcrypt.hash(this.password, +(process.env.HASH_SALT_ROUNDS || 10));\n\t}"));
    code.push(String::from("\tasync isPasswordSame(password: string): Promise<boolean> {\n\t\treturn this.password === await bcrypt.hash(password, +(process.env.HASH_SALT_ROUNDS || 10));\n\t}"));
    code.push(String::from("}"));
    code.join("\n")
}

fn generate_app_model(model: &Model, config_file: &ConfigFile) -> String {
    let mut code = Vec::new();
    code.push(String::from("import { DbItem } from './core/db.item';"));
    code.push(String::from("export class ") + &first_letter_to_uppper_case(&model.name) + " extends DbItem {");
    for field in model.fields.iter() {
        code.push(String::from("\t") + &field.name + ": " + &map_field_to_ts_type(&field.data_type) + ";");
    }
    code.push(String::from("\tconstructor(data: any) {\n\t\tsuper(data);"));
    for field in model.fields.iter() {
        code.push(String::from("\t\tthis.") + &field.name + " = data." + &field.name + ";");
    }
    
    code.push(String::from("\t}"));
    if config_file.auth && model.name == String::from("user") {
        code.push(String::from("\tisValid(): boolean {\n\t\tif (!(this.name && this.email && this.password && this.role)) {\n\t\t\treturn false;\n\t\t}\n\t\tif (this.role === 'ADMIN' && this.email !== process.env.ADMIN_EMAIL) {\n\t\t\treturn false;\n\t\t}\n\t\treturn true;\n\t}"));
        code.push(String::from("\tasync hashPassword(): Promise<void> {\n\t\tthis.password = await bcrypt.hash(this.password, +(process.env.HASH_SALT_ROUNDS || 10));\n\t}\n\nasync isPasswordSame(password: string): Promise<boolean> {\n\t\treturn this.password === await bcrypt.hash(password, +(process.env.HASH_SALT_ROUNDS || 10));\n\t}"));  
    }
    code.push(String::from("}"));
    code.join("\n")
}

fn generate_db_item_model() -> String {
    let mut code = Vec::new();
    code.push(String::from("const uuidv4 = require('uuid/v4');\n"));
    code.push(String::from("export class DbItem {\n"));
    code.push(String::from("\tid: string | undefined;\n"));
    code.push(String::from("\tconstructor(data: any | undefined){\n\t\tif (typeof data !== 'undefined') {\n\t\t\tthis.id = data.id;\n\t\t}\n\t}"));
    code.push(String::from("\tgenerateId(): DbItem {\n\t\tthis.id = uuidv4();return this;\n\t}"));
    code.push(String::from("\tisEmpty(): boolean {\n\t\tfor (let data in this){\n\t\t\tif (typeof this[data] !== 'undefined'){\n\t\t\t\treturn false;\n\t\t\t}\n\t\t}\n\t\treturn true;\n\t}"));
    code.push(String::from("\tvaluesToString(): string {\n\t\tconst str: string[] = [];\n\t\tfor (let key of Object.keys(this)){\n\t\t\tif (typeof this[key] !== 'undefined'){\n\t\t\t\tif (key !== 'id') {\n\t\t\t\t\tif (typeof this[key] === 'boolean') {\n\t\t\t\t\t\tstr.push(key + ' = ' + ((this[key])? 1 : 0));\n\t\t\t\t\t} else {\n\t\t\t\t\t\tstr.push(key + ' = \\'' + this[key] + '\\'');\n\t\t\t\t\t}\n\t\t\t\t}\n\t\t\t}\n\t\t}\n\t\treturn str.join(', ');\n\t}"));
    code.push(String::from("\twhereSimilarString(): string {\n\t\tconst str: string[] = [];\n\t\tfor (let key of Object.keys(this)) {\n\t\t\tif (typeof this[key] !== 'undefined') {\n\t\t\t\tif (key === 'id') continue;\n\t\t\t\tif (typeof this[key] === 'boolean') {\n\t\t\t\t\tstr.push(key + ' = ' + ((this[key] ? 1 : 0)));\n\t\t\t\t\tcontinue;\n\t\t\t\t}\n\t\t\t\tstr.push('UPPER(' + key + ') LIKE UPPER(\\'%' + this[key] + '%\\')');\n\t\t\t}\n\t\t}\n\t\treturn str.join(', ');\n\t}"));
    code.push(String::from("\tlistKeys(): string{\n\t\tconst str: string[] = [];\n\t\tfor (let data of Object.keys(this)){\n\t\t\tif (typeof this[data] !== 'undefined'){\n\t\t\t\tstr.push(data);\n\t\t\t}\n\t\t}\n\t\treturn str.join(', ');\n\t}"));
    code.push(String::from("\twhereString(): string {\n\t\tconst str: string[] = [];\n\t\tfor (let data of Object.keys(this)){\n\t\t\tif (typeof this[data] !== 'undefined') {\n\t\t\t\tif (typeof this[data] === 'number') {\n\t\t\t\t\tstr.push(data + ' = ' + this[data] + '');\n\t\t\t\t} else if (typeof this[data] === 'boolean') {\n\t\t\t\t\tstr.push(data + ' = ' + ((this[data])? 1 : 0) + '');\n\t\t\t\t} else {\n\t\t\t\t\tstr.push(data + ' = \\'' + this[data] + '\\'');\n\t\t\t\t}\n\t\t\t}\n\t\t}\n\t\tif (str.length === 0) return '1';\n\t\treturn str.join(' AND ');\n\t}"));
    code.push(String::from("\tlistValues(): string{\n\t\tconst str: string[] = [];\n\t\tfor (let data of Object.keys(this)) {\n\t\t\tif (typeof this[data] !== 'undefined') {\n\t\t\t\tif (typeof this[data] === 'boolean') {\n\t\t\t\t\tstr.push('\\'' + ((this[data])? 1 : 0) + '\\'');\n\t\t\t\t} else {\n\t\t\t\t\tstr.push('\\'' + this[data] + '\\'');\n\t\t\t\t}\n\t\t\t}\n\t\t}\n\t\treturn str.join(', ');\n\t}"));
    code.push(String::from("\tupdateValues(updates: DbItem): void {\n\t\tfor (let key of Object.keys(updates)) {\n\t\t\tif (key !== 'id') this[key] = updates[key];\n\t\t}\n\t}"));
    code.push(String::from("}"));
    code.join("\n")
}

fn generate_response_model() -> String {
    let mut code: Vec<String> = Vec::new();
    code.push(String::from("import * as express from 'express';"));
    code.push(String::from("export class Response {"));
    code.push(String::from("\tprivate code: number;\n\tprivate message: string | undefined;"));
    code.push(String::from("\tconstructor(code: number, message: string | undefined) {\n\t\tthis.code = code;\n\t\tthis.message = message;\n\t}"));
    code.push(String::from("\tsetMessage(message: string): void {\n\t\tthis.message = message;\n\t}\n\tsetCode(code: number): void {\n\t\tthis.code = code;\n\t}"));
    code.push(String::from("\tgetCode(): number {\n\t\treturn this.code;\n\t}"));
    code.push(String::from("\tsend(expressResponse: express.Response): void {\n\t\texpressResponse.status(this.code);\n\t\texpressResponse.send(this);\n\t}"));
    code.push(String::from("}"));
    code.join("\n")
}

fn generate_error_response_model() -> String {
    let mut code: Vec<String> = Vec::new();
    code.push(String::from("import { Response } from './response';"));
    code.push(String::from("export class ErrorResponse extends Response {"));
    code.push(String::from("\tprivate error?: any;"));
    code.push(String::from("\tconstructor(code?: number, message?: string, error?: string) {\n\t\tsuper(code || 500, message || 'Error occured');\n\t\tthis.error = error;\n\t}"));
    code.push(String::from("\tsetError(error: any): ErrorResponse {\n\t\tthis.error = error;\n\t\treturn this;\n\t}"));
    code.push(String::from("}"));
    code.join("\n")
}

fn generate_success_response_model() -> String {
    let mut code: Vec<String> = Vec::new();
    code.push(String::from("import { Response } from './response';"));
    code.push(String::from("export class SuccessResponse extends Response {"));
    code.push(String::from("\tprivate data?: any;"));
    code.push(String::from("\tconstructor(code?: number, message?: string, data?: any) {\n\t\tsuper(code || 200, message || 'Success');\n\t\tthis.data = data;\n\t}"));
    code.push(String::from("\tsetData(data: any): SuccessResponse {\n\t\tthis.data = data;\n\t\treturn this;\n\t}"));
    code.push(String::from("}"));
    code.join("\n")
}
