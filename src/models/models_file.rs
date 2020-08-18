#[derive(Serialize, Deserialize)]
pub struct ModelFile {
    models: Vec<Model>
}


#[derive(Serialize, Deserialize)]
struct Model {
    #[serde(default="Model::name_default")]
    name: String,
    crud: Crud,
    fields: Vec<Field>
}

#[derive(Serialize, Deserialize)]
struct Crud {
    create: bool,
    create_auth: bool,
    read: bool,
    read_auth: bool,
    update: bool,
    update_auth: bool,
    delete: bool,
    delete_auth: bool,
}

#[derive(Serialize, Deserialize)]
struct Field {
    name: String,
    data_type: String
}

impl ModelFile {
    pub fn new () -> ModelFile {
        ModelFile {
            models: vec!(Model {
                name: String::from("test"),
                crud: Crud {
                    create: true,
                    create_auth: true,
                    read: true,
                    read_auth: true,
                    update: true,
                    update_auth: true,
                    delete: true,
                    delete_auth: true
                },
                fields: vec!(
                    Field {
                        name: String::from("test1"),
                        data_type: String::from("VARCHAR")
                    },
                    Field {
                        name: String::from("test2"),
                        data_type: String::from("VARCHAR")
                    },
                    Field {
                        name: String::from("test3"),
                        data_type: String::from("VARCHAR")
                    },
                )
            })
        }
    }
}

impl Model {
    fn name_default() -> String {
        String::from("default_test")
    }
}