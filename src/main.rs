mod cli;

use structopt::StructOpt;
use cli::CliOptions;
use cli::Task;

fn main() {
    let options = CliOptions::from_args();

    let task: Task = match &options.command as &str {
        "init" => Task::Init,
        "build" => Task::Build,
        _e @ _ => Task::None,
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

fn init_project(_: CliOptions) -> () {

}


fn build_project(_: CliOptions) -> () {
    
}

