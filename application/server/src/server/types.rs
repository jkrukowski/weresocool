use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Language {
    pub language: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PrintLanguage {
    pub language: String,
    pub print_type: String,
}
