use std::error::Error;
use std::str::FromStr;

use crate::{list::database_model::ListDatabaseModel, shared::database_error::MyDBError};
use futures::StreamExt;
use mongodb::{
    bson::{doc, oid::ObjectId},
    options::{FindOneAndUpdateOptions, FindOptions, ReturnDocument},
    Collection,
};

pub async fn fetch_single_list(
    collection: &Collection<ListDatabaseModel>,
    id: &String,
) -> Result<ListDatabaseModel, Box<dyn Error>> {
    let id_as_object = ObjectId::from_str(&id).map_err(|_| MyDBError::NotFoundError(id.clone()))?;

    let filter = doc! { "_id": id_as_object };

    match collection.find_one(filter, None).await {
        Ok(Some(doc)) => Ok(doc),
        Ok(None) => Err(Box::new(MyDBError::NotFoundError(id.clone()))),
        Err(e) => Err(Box::new(MyDBError::MongoQueryError(e))),
    }
}
pub async fn fetch_lists(
    collection: &Collection<ListDatabaseModel>,
    limit: i64,
    page: i64,
) -> Result<Vec<ListDatabaseModel>, Box<dyn Error>> {
    let find_options = FindOptions::builder()
        .limit(limit)
        .skip(u64::try_from((page - 1) * limit).unwrap())
        .build();

    let mut cursor = collection
        .find(None, find_options)
        .await
        .map_err(MyDBError::MongoQueryError)?;

    let mut db_result: Vec<ListDatabaseModel> = Vec::new();
    while let Some(doc) = cursor.next().await {
        match doc {
            Ok(item) => db_result.push(item),
            Err(e) => {
                println!("Error processing document: {}", e);
                continue;
            }
        }
    }

    println!("fetch_lists returns {:?}", db_result);

    Ok(db_result)
}

pub async fn create_list(
    collection: &Collection<ListDatabaseModel>,
    list: &ListDatabaseModel,
) -> Result<ListDatabaseModel, Box<dyn Error>> {
    // Insert into collection
    let result = match collection.insert_one(list, None).await {
        Ok(result) => result,
        Err(e) => {
            if e.to_string()
                .contains("E11000 duplicate key error collection")
            {
                return Err(Box::new(MyDBError::MongoDuplicateError(e)));
            }
            return Err(Box::new(MyDBError::MongoQueryError(e)));
        }
    };
    let inserted_id_string: String = result.inserted_id.as_object_id().unwrap().to_hex();
    let filter = doc! { "_id": result.inserted_id.as_object_id().unwrap() };
    let inserted_doc = match collection.find_one(filter, None).await {
        Ok(Some(doc)) => doc,
        Ok(None) => {
            return Err(Box::new(MyDBError::NotFoundError(
                inserted_id_string.clone(),
            )))
        }
        Err(e) => return Err(Box::new(MyDBError::MongoQueryError(e))),
    };

    Ok(inserted_doc)
}
pub async fn edit_list(
    collection: &Collection<ListDatabaseModel>,
    list: &ListDatabaseModel,
) -> Result<ListDatabaseModel, Box<dyn Error>> {
    let filter = doc! { "_id": list._id.clone() };
    let update_doc = bson::to_document(list).unwrap();
    let update = doc! { "$set": update_doc};

    let options = FindOneAndUpdateOptions::builder()
        .return_document(ReturnDocument::After)
        .build();

    match collection
        .find_one_and_update(filter, update, options)
        .await
    {
        Ok(Some(doc)) => Ok(doc),
        Ok(None) => Err(Box::new(MyDBError::NotFoundError(list._id.to_string()))),
        Err(e) => Err(Box::new(MyDBError::MongoQueryError(e))),
    }
}

pub async fn delete_list(
    collection: &Collection<ListDatabaseModel>,
    id: &str,
) -> Result<(), Box<dyn Error>> {
    // convert string into ObjectId
    let id_as_object =
        ObjectId::from_str(id).map_err(|_| MyDBError::NotFoundError(id.to_string()))?;
    let filter = doc! {"_id": id_as_object };

    match collection.delete_one(filter, None).await {
        Ok(result) => match result.deleted_count {
            0 => Err(Box::new(MyDBError::NotFoundError(id.to_string()))),
            _ => Ok(()),
        },
        Err(e) => Err(Box::new(MyDBError::MongoQueryError(e))),
    }
}
