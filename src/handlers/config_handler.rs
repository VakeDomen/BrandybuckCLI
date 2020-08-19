use crate::models::cli_options::CliOptions;
use crate::models::config_file::ConfigFile;
use crate::models::models_file::ModelFile;
use crate::handlers::file_handler::{generate_folder, write_file};

static CONFIG_FILE_NAME: &'static str = "./brandybuck.config.json";
static MODELS_FILE_NAME: &'static str = "./brandybuck.models.json";

pub fn generate_new_configurations(options: CliOptions) -> () {
    generate_model_template(&options);
    generate_configuration_template(&options);
}

fn generate_configuration_template(_options: &CliOptions) -> () {
    let configuration = ConfigFile::new();
    let mut serialized = serde_json::to_string_pretty(&configuration).unwrap();
    write_file(
        &mut serialized,
        CONFIG_FILE_NAME.to_string()
    );
}

fn generate_model_template(_options: &CliOptions) -> () {
    let model = ModelFile::new();
    let mut serialized = serde_json::to_string_pretty(&model).unwrap();
    write_file(
        &mut serialized,
        MODELS_FILE_NAME.to_string()
    );
}
