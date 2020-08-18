extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

mod models;
mod handlers;

use handlers::config_handler::generate_new_configurations;
use structopt::StructOpt;
use models::cli::{CliOptions, Task};
use log::{info};

fn main() {
    let options = CliOptions::from_args();

    let task: Task = match &options.command as &str {
        "init" => Task::Init,
        "build" => Task::Build,
        _ => Task::None,
    };
    execute_task(task, options);
}

fn execute_task(task: Task, options: CliOptions) -> () {
    match task {
        Task::Init => init_project(options),
        Task::Build => build_project(options),
        Task::None => println!("No valid command specified! You should use the 'init' command to start a new project or 'build' to build from the cofiguration")
    }
}

fn init_project(options: CliOptions) -> () {
    generate_new_configurations(options);
    info!("Done!");
}

fn build_project(_: CliOptions) -> () {
    info!("Done!");
}

