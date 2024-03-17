use serde::Deserialize;

#[derive(Deserialize)]
pub struct NewItemRequestModel {
    pub listId: String,
    pub name: String,
    pub state: Option<String>,
    pub description: Option<String>,
    pub dueDate: Option<String>
}
#[derive(Deserialize)]
pub struct UpdateItemRequestModel {
    pub id: String,
    pub listId: String,
    pub name: String,
    pub state: String,
    pub description: Option<String>,
    pub dueDate: Option<String>,
    pub completedDate: Option<String>,
    pub createdDate: Option<String>,
    pub updatedDate: Option<String>

}