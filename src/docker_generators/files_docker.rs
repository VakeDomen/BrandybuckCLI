use crate::models::brandybuck_config_file::ConfigFile;
use crate::ts_generators::files_dotenv::{generate_docker_environment, generate_dotenv_file, generate_dotenv_sample_file};

pub fn generate_dockerization_files(config_file: &ConfigFile) -> Vec<(String, String)> {
    let mut files: Vec<(String, String)> = Vec::new();
    files.push((String::from("Dockerfile"), genrate_dockerfile_file(config_file)));
    files.push((String::from("docker-compose.yml"), genrate_docker_compose_file(config_file)));
    files.push((String::from("server.entrypoint.sh"), genrate_entrypoint_file(config_file)));
    files.push((String::from(".env"), generate_dotenv_file(config_file)));
    files.push((String::from(".env.sample"), generate_dotenv_sample_file(config_file)));
    files
}

fn genrate_dockerfile_file(config_file: &ConfigFile) -> String {
    let mut code = Vec::new();
    code.push(String::from("FROM node:13-alpine"));
    code.push(String::from("WORKDIR /server"));
    code.push(String::from("COPY app ."));
    code.push(String::from("COPY server.entrypoint.sh ."));
    code.push(String::from("RUN npm install && chmod 777 ./server.entrypoint.sh"));
    code.push(String::from("EXPOSE 3000"));
    code.push(String::from("ENTRYPOINT [\"sh\", \"./server.entrypoint.sh\"]"));
    code.join("\n")
}

fn genrate_docker_compose_file(config_file: &ConfigFile) -> String {
    let mut code = Vec::new();
    code.push(String::from("version: '3.2'"));
    code.push(String::from("services:\n  server:\n    restart: \"unless-stopped\""));
    code.push(String::from("    build:\n      context: .\n      dockerfile: ./Dockerfile"));
    code.push(String::from("    environment:\n") + &generate_docker_environment(config_file));
    code.push(String::from("    ports:\n      - \"3000:3000\""));
    code.join("\n")
}

fn genrate_entrypoint_file(config_file: &ConfigFile) -> String {
    let mut code = Vec::new();
    code.push(String::from("#!/bin/bash"));
    code.push(String::from("echo \"Starting service in $PWD\""));
    code.push(String::from("npm run prod"));
    code.join("\n")
}

