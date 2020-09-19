use crate::models::brandybuck_config_file::{ConfigFile, Docker, DockerConfig, Traefik, TraefikConfig};
use crate::ts_generators::files_dotenv::{generate_docker_environment, generate_dotenv_file, generate_dotenv_sample_file};

pub fn generate_docker_files(config_file: &ConfigFile) -> Vec<(String, String)> {
    let mut files: Vec<(String, String)> = Vec::new();
    files.push((String::from("Dockerfile"), genrate_dockerfile_file(config_file)));
    files.push((String::from("docker-compose.yml"), genrate_docker_compose_file(config_file)));
    files.push((String::from("server.entrypoint.sh"), genrate_entrypoint_file(config_file)));
    files.push((String::from(".env"), generate_dotenv_file(config_file, true)));
    files.push((String::from(".env.sample"), generate_dotenv_sample_file(config_file, true)));
    files
}

fn genrate_dockerfile_file(config_file: &ConfigFile) -> String {
    let mut code = Vec::new();
    code.push(String::from("FROM node:13-alpine"));
    code.push(String::from("WORKDIR /server"));
    code.push(String::from("COPY app ."));
    code.push(String::from("COPY server.entrypoint.sh ."));
    code.push(String::from("RUN npm install && chmod 777 ./server.entrypoint.sh"));
    code.push(format!("EXPOSE {PORT}", PORT = config_file.port));
    code.push(String::from("ENTRYPOINT [\"sh\", \"./server.entrypoint.sh\"]"));
    code.join("\n")
}

fn genrate_docker_compose_file(config_file: &ConfigFile) -> String {
    let mut code = Vec::new();
    let docker_port = match &config_file.docker {
        Docker::Bool(b) => config_file.port,
        Docker::Config(config) => config.port
    };
    code.push(String::from("version: '3.2'"));
    code.push(String::from("services:\n  server:\n    restart: \"unless-stopped\""));
    code.push(String::from("    build:\n      context: .\n      dockerfile: ./Dockerfile"));
    code.push(String::from("    environment:\n") + &generate_docker_environment(config_file));
    let traefik_config_option: Option<TraefikConfig> = config_file.get_traefik2_conf();
    match traefik_config_option {
        Some(_) => {
            code.push(generate_traefik2(config_file));
            code.push(generate_network_imports(config_file));
        },
        None => code.push(format!("    ports:\n      - \"{docker_port}:{be_port}\"", docker_port = docker_port, be_port = config_file.port))
    }
    
    code.join("\n")
}

fn generate_network_imports(config_file: &ConfigFile) -> String {
    let traefik_config_option: Option<TraefikConfig> = config_file.get_traefik2_conf();
    match traefik_config_option {
        Some(traefik_config) => {
            let mut code = Vec::new();
            code.push(String::from("networks:"));
            code.push(String::from("  ") + &traefik_config.proxy_network_name + ":");
            code.push(String::from("    external: true"));
            code.join("\n")
        },
        None => String::from("")
    }
}

fn generate_traefik2(config_file: &ConfigFile) -> String {
    let traefik_config: Option<TraefikConfig> = config_file.get_traefik2_conf();
    match traefik_config {
        Some(conf) => {
            let mut code = Vec::new();
            code.push(format!("    networks:\n      - {network}", network = conf.proxy_network_name));
            code.push(generate_traefik2_labels(config_file));
            code.join("\n")
        },
        None => String::from("")
    }
}

fn generate_traefik2_labels(config_file: &ConfigFile) -> String {
    let traefik_config_option: Option<TraefikConfig> = config_file.get_traefik2_conf();
    let mut code = Vec::new();
    if let Some(traefik_config) = traefik_config_option {
        code.push(String::from("    labels:"));
        code.push(format!("      - traefik.enable = true"));
        code.push(format!("      - traefik.http.routers.{name}.rule=Host({domain_env})", name = traefik_config.container_name, domain_env = String::from("`${DOMAIN}`")));
        code.push(format!("      - traefik.http.routers.{name}.entrypoints = {entrypoint}", name = traefik_config.container_name, entrypoint = traefik_config.entrypoint_name));
        code.push(format!("      - traefik.http.routers.{name}.tls = true", name = traefik_config.container_name));
        code.push(format!("      - traefik.http.routers.{name}.tls.certresolver = {certresolver}", name = traefik_config.container_name, certresolver = traefik_config.certresolver_name));
        code.push(format!("      - traefik.http.services.{name}.loadbalancer.server.port = {port}", name = traefik_config.container_name, port = config_file.port));
    }
    code.join("\n")
}

fn genrate_entrypoint_file(config_file: &ConfigFile) -> String {
    let mut code = Vec::new();
    code.push(String::from("#!/bin/bash"));
    code.push(String::from("echo \"Starting service in $PWD\""));
    code.push(String::from("npm run prod"));
    code.join("\n")
}

