use crate::models::brandybuck_config_file::ConfigFile;

pub fn generate_auth_files(config_file: &ConfigFile) -> Vec<(String, String)> {
    let mut files: Vec<(String, String)> = Vec::new();
    files.push((String::from("app/src/auth/local.auth.ts"), generate_local_auth(config_file)));
    files
}

fn generate_local_auth(config_file: &ConfigFile) -> String {
    let mut code = Vec::new();
    code.push(generate_imports());
    code.push(generate_login());
    code.push(generate_register());
    code.push(generate_util_functions());
    code.push(generate_token_verification_middleware());
    code.join("\n")
}

fn generate_login() -> String {
    let mut code = Vec::new();
    code.push(String::from("export async function login(reqest: express.Request): Promise<Response> {"));
    code.push(String::from("\tconst credentials: LocalCredentials = extractCredentials(reqest);"));
    code.push(String::from("\tif (!validCredentials(credentials)) {"));
    code.push(String::from("\t\treturn new ErrorResponse(400, 'Invalid credentials');"));
    code.push(String::from("\t}"));
    code.push(String::from("\tconst users = await fetch(config.tables.users, new User({ email: credentials.email }));"));
    code.push(String::from("\tif (!users || users.length !== 1) {"));
    code.push(String::from("\t\treturn new ErrorResponse(401, 'Unauthorized!');"));
    code.push(String::from("\t}"));
    code.push(String::from("\tconst user = new User(users.pop());"));
    code.push(String::from("\tconsole.log(user);"));
    code.push(String::from("\tif(!user.isPasswordSame(credentials.password)) {"));
    code.push(String::from("\t\treturn new ErrorResponse(401, 'Unauthorized!');"));
    code.push(String::from("\t}"));
    code.push(String::from("\tconst token = await jwt.sign(JSON.stringify(user), process.env.JWT_SECRET);"));
    code.push(String::from("\treturn new SuccessResponse().setData(token);"));
    code.push(String::from("}"));
    code.join("\n")
}

fn generate_register() -> String {
    let mut code = Vec::new();
    code.push(String::from("export async function register(request: express.Request): Promise<Response> {"));
    code.push(String::from("\tconst user = new User(request.body);"));
    code.push(String::from("\tconst existingUser = await fetch(config.tables.users, new User({ email: user.email }));"));
    code.push(String::from("\tif (existingUser && existingUser.length > 0) {"));
    code.push(String::from("\t\treturn new ErrorResponse(409, 'User with that email already exists!');"));
    code.push(String::from("\t}"));
    code.push(String::from("\tif (!user.isValid()) {"));
    code.push(String::from("\t\treturn new ErrorResponse(400, 'Invalid credentials!');"));
    code.push(String::from("\t}"));
    code.push(String::from("\tuser.generateId();"));
    code.push(String::from("\tawait user.hashPassword();"));
    code.push(String::from("\tawait insert(config.tables.users, user);"));
    code.push(String::from("\treturn new SuccessResponse(201, 'User registered');"));
    code.push(String::from("}"));
    code.join("\n")
}

fn generate_token_verification_middleware() -> String {
    let mut code = Vec::new();
    code.push(String::from("export async function verifyTokenMiddleware(req: express.Request, resp: express.Response, next: any): Promise<void> {"));
    code.push(String::from("\tconst extraction: string | ErrorResponse = extractToken(req);"));
    code.push(String::from("\tlet token: string = '';"));
    code.push(String::from("\tif (typeof extraction !== 'string') {"));
    code.push(String::from("\t\tnew ErrorResponse(401, 'Unauthorized').send(resp);"));
    code.push(String::from("\t} else {"));
    code.push(String::from("\t\ttoken = extraction;"));
    code.push(String::from("\t}"));
    code.push(String::from("\tconst userJSON: string | null | void =  await verifyToken(token)"));
    code.push(String::from("\t.catch(() => new ErrorResponse(401, 'Unauthorized').send(resp));"));
    code.push(String::from("\tif (!userJSON) {"));
    code.push(String::from("\t\tnew ErrorResponse(401, 'Unauthorized').send(resp);"));
    code.push(String::from("\t} else {"));
    code.push(String::from("\t\tconst user: User = new User(userJSON);"));
    code.push(String::from("\t\tconst loggedUser = await fetch(config.tables.users, user);"));
    code.push(String::from("\t\tObject.assign(req, { token: token, loggedUser: loggedUser.pop() });"));
    code.push(String::from("\t\tnext();"));
    code.push(String::from("\t}"));
    code.push(String::from("}"));
    code.join("\n")
}

fn generate_imports() -> String {
    let mut code = Vec::new();
    code.push(String::from("import { LocalCredentials } from '../models/core/local.credentials';"));
    code.push(String::from("import { SuccessResponse } from '../models/core/success.response';"));
    code.push(String::from("import { Response } from '../models/core/response';"));
    code.push(String::from("import { ErrorResponse } from '../models/core/error.response';"));
    code.push(String::from("import { User } from '../models/user.item';"));
    code.push(String::from("import { fetch, insert } from '../db/database.handler';"));
    code.push(String::from("import * as express from 'express';"));
    code.push(String::from("import * as config from '../db/database.config.json';"));
    code.push(String::from("import * as jwt from 'jsonwebtoken';"));
    code.push(String::from("\n"));
    code.join("\n")
}

fn generate_util_functions() -> String {
    let mut code = Vec::new();
    code.push(String::from("async function verifyToken(token): Promise<string | null> {"));
    code.push(String::from("\treturn await jwt.verify(token, process.env.JWT_SECRET);"));
    code.push(String::from("}"));
    code.push(String::from(""));
    code.push(String::from("function validCredentials(crednetials: LocalCredentials): boolean {"));
    code.push(String::from("\tif (!crednetials.email || !crednetials.password) {"));
    code.push(String::from("\t\treturn false;"));
    code.push(String::from("\t}"));
    code.push(String::from("\treturn true;"));
    code.push(String::from("}"));
    code.push(String::from(""));
    code.push(String::from("function extractToken(request: express.Request): string | ErrorResponse {"));
    code.push(String::from("\tconst extraction: string | string[] | undefined = request.headers['authorization'];"));
    code.push(String::from("\tif (typeof extraction !== 'string') {"));
    code.push(String::from("\t\treturn new ErrorResponse(401, 'Unauthorized');"));
    code.push(String::from("\t} else {"));
    code.push(String::from("\t\treturn extraction.replace('Bearer ', '');"));
    code.push(String::from("\t}"));
    code.push(String::from("}"));
    code.push(String::from(""));
    code.push(String::from("function extractCredentials(request: express.Request): LocalCredentials {"));
    code.push(String::from("\tconst password: string = request.body['password'];"));
    code.push(String::from("\tconst email: string = request.body['email'];"));
    code.push(String::from("\treturn new LocalCredentials(email, password);"));
    code.push(String::from("}"));
    code.join("\n")
}