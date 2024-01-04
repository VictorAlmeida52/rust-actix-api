use serde::{ Deserialize, Serialize };

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateTaskSchema {
    pub title: String,
    pub content: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct FilterOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateTaskSchema {
    pub title: Option<String>,
    pub content: Option<String>,
}