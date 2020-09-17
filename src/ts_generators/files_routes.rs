use crate::models::brandybuck_config_file::ConfigFile;
use crate::models::brandybuck_models_file::{ModelFile, Model};
use crate::helpers::util_helper::first_letter_to_uppper_case;


pub fn generate_routes_files(config_file: &ConfigFile, models_file: &ModelFile) -> Vec<(String, String)> {
    let mut files: Vec<(String, String)> = Vec::new();
    for model in models_file.models.iter() {
        if model.crud.create || model.crud.read || model.crud.update || model.crud.delete {
            let code = generate_route_file(model, config_file);
            files.push((
                model.name.clone() + ".ts",
                code
            ));
        }
    }
    files
}

fn generate_route_file(model: &Model, config_file: &ConfigFile) -> String {
    let mut code = Vec::new();
    code.push(generate_imports(model, config_file));
    code.push(generate_initialisation());
    code.push(generate_crud(model, config_file));
    code.join("\n")
}

fn generate_crud(model: &Model, config_file: &ConfigFile) -> String {
    let mut code = Vec::new();
    if model.crud.read {
        code.push(generate_read(model, config_file));
    }
    if model.crud.create {
        code.push(generate_insert(model, config_file));
    }
    if model.crud.update {
        code.push(generate_update(model, config_file));
    }
    if model.crud.delete {
        code.push(generate_delete(model, config_file));
    }
    code.join("\n")
}

fn generate_update(model: &Model, config_file: &ConfigFile) -> String {
    let mut code = Vec::new();
    let route = String::from(&model.name) + "/";
    let auth: String = if config_file.auth { String::from("verifyTokenMiddleware, ") } else { String::from("") };
    code.push(format!("router.patch('/{route}', {auth} async (req: express.Request, resp: express.Response) => {sq}", route = route, auth = auth, sq = String::from("{")));
    code.push(format!("\tconst {model} = await update(conf.tables.{model}, new {upper_case_model}(req.body)).catch(err => {sq}", model = &model.name, upper_case_model = &first_letter_to_uppper_case(&model.name), sq = String::from("{")));
    code.push(format!("\t\treturn new ErrorResponse().setError(err).send(resp);\n\t{sq});", sq = String::from("}")));
    code.push(format!("\treturn new SuccessResponse().setData({model}).send(resp);\n{sq});\n", model = &model.name, sq = String::from("}")));
    code.join("\n")
}

fn generate_delete(model: &Model, config_file: &ConfigFile) -> String {
    let mut code = Vec::new();
    let route = String::from(&model.name) + "/";
    let auth: String = if config_file.auth { String::from("verifyTokenMiddleware, ") } else { String::from("") };
    code.push(format!("router.delete('/{route}:id', {auth} async (req: express.Request, resp: express.Response) => {sq}", route = route, auth = auth, sq = String::from("{")));
    code.push(format!("\tawait deleteItem(conf.tables.{model}, new {upper_case_model}(req.params['id'])).catch(err => {sq}", model = &model.name, upper_case_model = &first_letter_to_uppper_case(&model.name), sq = String::from("{")));
    code.push(format!("\t\treturn new ErrorResponse().setError(err).send(resp);\n\t{sq});", sq = String::from("}")));
    code.push(format!("\treturn new SuccessResponse().send(resp);\n{sq});\n", sq = String::from("}")));
    code.join("\n")
}

fn generate_insert(model: &Model, config_file: &ConfigFile) -> String {
    let mut code = Vec::new();
    let route = String::from(&model.name) + "/";
    let auth: String = if config_file.auth { String::from("verifyTokenMiddleware, ") } else { String::from("") };
    code.push(format!("router.post('/{route}', {auth} async (req: express.Request, resp: express.Response) => {sq}", route = route, auth = auth, sq = String::from("{")));
    code.push(format!("\tconst {model}s = await insert(conf.tables.{model}, new {upper_case_model}(req.body).generateId()).catch(err => {sq}", model = &model.name, upper_case_model = &first_letter_to_uppper_case(&model.name), sq = String::from("{")));
    code.push(format!("\t\treturn new ErrorResponse().setError(err).send(resp);\n\t{sq});", sq = String::from("}")));
    code.push(format!("\treturn new SuccessResponse().setData({model}s).send(resp);\n{sq});\n", model = &model.name, sq = String::from("}")));
    code.join("\n")
}

fn generate_read(model: &Model, config_file: &ConfigFile) -> String {
    let mut code = Vec::new();
    let route = String::from(&model.name) + "/";
    let auth: String = if config_file.auth { String::from("verifyTokenMiddleware, ") } else { String::from("") };
    code.push(format!("router.get('/{route}', {auth} async (req: express.Request, resp: express.Response) => {sq}", route = route, auth = auth, sq = String::from("{")));
    code.push(format!("\tconst {model}s = await fetch(conf.tables.{model}, new {upper_case_model}(req.query)).catch(err => {sq}", model = &model.name, upper_case_model = &first_letter_to_uppper_case(&model.name), sq = String::from("{")));
    code.push(format!("\t\tconsole.log(err);return new ErrorResponse().setError(err).send(resp);\n\t{sq});", sq = String::from("}")));
    code.push(format!("\treturn new SuccessResponse().setData({model}s).send(resp);\n{sq});\n", model = &model.name, sq = String::from("}")));
    code.join("\n")
}

fn generate_initialisation() -> String {
    let mut code = Vec::new();
    code.push("const router: express.Router = express.Router();");
    code.push("module.exports = router;\n");
    code.join("\n")
}

fn generate_imports(model: &Model, config_file: &ConfigFile) -> String {
    let mut code = Vec::new();
    if config_file.auth {
        code.push(String::from("import { verifyTokenMiddleware } from '../auth/local.auth.ts';"));
    }
    code.push(String::from("import { ErrorResponse } from './../models/core/error.response';\nimport { SuccessResponse } from './../models/core/success.response';"));
    code.push(String::from("import * as express from 'express';"));
    code.push(String::from("import { ") + &crud_imports(&model) + " } from '../db/database.handler';\nimport * as conf from '../db/database.config.json';");
    code.push(String::from("import { ") + &first_letter_to_uppper_case(&model.name) + " } from '../models/" + &model.name + ".item';\n");
    code.join("\n")
}

fn crud_imports(model: &Model) -> String {
    let mut imports = Vec::new();
    if model.crud.read {
        imports.push(String::from("fetch"));
    }
    if model.crud.create {
        imports.push(String::from("insert"));
    }
    if model.crud.update {
        imports.push(String::from("update"));
    }
    if model.crud.delete {
        imports.push(String::from("deleteItem"));
    }
    imports.join(", ")
}