use bson::{Bson, DateTime as BsonDateTime};
use mongodb::bson::{self, doc, oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use std::str::FromStr;

#[allow(non_snake_case)]
#[derive(PartialEq, Serialize, Deserialize, Debug, Clone)]
pub enum TodoItemState {
    #[serde(rename = "todo")]
    Todo,
    #[serde(rename = "inprogress")]
    InProgress,
    #[serde(rename = "done")]
    Done,
}
impl TodoItemState {
    pub fn to_str(state: &Self) -> &'static str {
        match *state {
            TodoItemState::Todo => "todo",
            TodoItemState::InProgress => "inprogress",
            TodoItemState::Done => "done",
        }
    }
    pub fn from_string(s: &str) -> Self {
        println!("state from_string: {:?}", s);

        match s {
            "todo" => TodoItemState::Todo,
            "inprogress" => TodoItemState::InProgress,
            "done" => TodoItemState::Done,
            _ => TodoItemState::Todo,
        }
    }
}
#[allow(non_snake_case)]
#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ItemDatabaseModel {
    pub _id: ObjectId,
    pub listId: ObjectId,
    pub name: String,
    pub state: TodoItemState,
    pub description: Option<String>,
    pub dueDate: Option<DateTime>,
    pub completedDate: Option<DateTime>,
    pub createdDate: Option<DateTime>,
    pub updatedDate: Option<DateTime>,
}
impl ItemDatabaseModel {
    pub fn string_to_optional_datetime2(s: &String) -> Option<BsonDateTime> {
        match BsonDateTime::parse_rfc3339_str(s) {
            Ok(dt) => Some(dt.into()),
            Err(_) => None,
        }
    }
    // pub fn string_to_datetime(s: &String) -> BsonDateTime {
    //     let new_object = BsonDateTime::parse_rfc3339_str(s).unwrap();
    //     return new_object;
    // }
    #[allow(non_snake_case)]
    pub fn new(
        listId: String,
        name: String,
        state: String,
        description: Option<String>,
        dueDate: Option<String>,
    ) -> Self {
        let now = bson::DateTime::now();
        let due_date = match dueDate {
            Some(s) => ItemDatabaseModel::string_to_optional_datetime2(&s),
            None => None,
        };

        let state = TodoItemState::from_string(&state);

        Self {
            _id: ObjectId::new(),
            listId: ObjectId::from_str(&listId).unwrap(),
            name: name,
            state: state,
            description: description,
            dueDate: due_date,
            completedDate: None,
            createdDate: Some(now),
            updatedDate: None,
        }
    }
    #[allow(non_snake_case)]
    pub fn update(
        id: String,
        listId: String,
        name: String,
        state: String,
        description: Option<String>,
        dueDate: Option<String>,
        completedDate: Option<String>,
        createdDate: Option<String>,
        _: Option<String>,
    ) -> Self {
        let now = bson::DateTime::now();

        let due_date = match dueDate {
            Some(s) => ItemDatabaseModel::string_to_optional_datetime2(&s),
            None => None,
        };

        let created_date = match createdDate {
            Some(s) => ItemDatabaseModel::string_to_optional_datetime2(&s),
            None => None,
        };

        let state = state.to_lowercase();
        let state_enum = TodoItemState::from_string(&state);
        let completed_date = if state_enum == TodoItemState::Done {
            Some(now)
        } else {
            match completedDate {
                Some(s) => ItemDatabaseModel::string_to_optional_datetime2(&s),
                None => None,
            }
        };

        Self {
            _id: ObjectId::from_str(&id).unwrap(),
            listId: ObjectId::from_str(&listId).unwrap(),
            name: name,
            state: state_enum,
            description: description,
            dueDate: due_date,
            completedDate: completed_date,
            createdDate: created_date,
            updatedDate: Some(now),
        }
    }
    pub fn read(&self) -> Bson {
        // convert _id and listId from ObjectId to string
        let id = self._id.to_hex();
        let list_id = self.listId.to_hex();
        let name = self.name.clone();

        let state_obj: TodoItemState = self.state.clone();
        let state = TodoItemState::to_str(&state_obj);

        let description = self.description.clone();

        let due_date = self
            .dueDate
            .map(|date| date.try_to_rfc3339_string().unwrap());
        let completed_date = self
            .completedDate
            .map(|date| date.try_to_rfc3339_string().unwrap());
        let updated_date = self
            .updatedDate
            .map(|date| date.try_to_rfc3339_string().unwrap());
        let created_date = self
            .createdDate
            .map(|date| date.try_to_rfc3339_string().unwrap());

        let doc = doc! {
            "id": id,
            "listId": list_id,
            "name": name,
            "state": state,
            "description": description,
            "dueDate": due_date,
            "completedDate": completed_date,
            "createdDate": created_date,
            "updatedDate": updated_date,
        };

        Bson::Document(doc)
    }
    pub fn to_response_body(&self) -> axum::body::Body {
        let body = self.read();
        axum::body::Body::from(serde_json::to_string(&body).unwrap())
    }
}
