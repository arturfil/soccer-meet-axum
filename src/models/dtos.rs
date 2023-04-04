use serde::{Serialize, Deserialize};

#[derive(Deserialize, Debug, Default)]
pub struct FilterOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,    
}

#[derive(Deserialize, Debug)]
pub struct ParamOptions {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateGameSchema {
    pub field_name: String,
    pub address: String,
    pub day: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateGameSchema {
    pub field_name: Option<String>,
    pub address: Option<String>,
    pub day: Option<String>,
}
