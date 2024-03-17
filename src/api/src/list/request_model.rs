use serde::Deserialize;

#[derive(Deserialize)]
pub struct NewListRequestModel {
    pub name: String,
}
#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct UpdateListRequestModel {
    pub id: Option<String>,
    pub name: String,
    pub createdDate: String,
    pub updatedDate: Option<String>,
}
