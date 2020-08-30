#[derive(Serialize, Deserialize)]
pub struct ModelFile {
    pub models: Vec<Model>
}


#[derive(Serialize, Deserialize)]
pub struct Model {
    #[serde(default="Model::name_default")]
    pub name: String,
    pub crud: Crud,
    pub fields: Vec<Field>
}

#[derive(Serialize, Deserialize)]
pub struct Crud {
    pub create: bool,
    pub create_auth: bool,
    pub read: bool,
    pub read_auth: bool,
    pub update: bool,
    pub update_auth: bool,
    pub delete: bool,
    pub delete_auth: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Field {
    pub name: String,
    pub data_type: String,
    pub null: bool
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
                        data_type: String::from("VARCHAR"),
                        null: false
                    },
                    Field {
                        name: String::from("test2"),
                        data_type: String::from("VARCHAR"),
                        null: false
                    },
                    Field {
                        name: String::from("test3"),
                        data_type: String::from("VARCHAR"),
                        null: false
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