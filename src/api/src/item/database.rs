use std::error::Error;
use std::str::FromStr;

use futures::StreamExt;
use mongodb::{
    bson,
    bson::{doc, oid::ObjectId},
    options::{FindOneAndUpdateOptions, FindOptions, ReturnDocument},
    Collection,
};

use crate::{
    item::database_model::{ItemDatabaseModel, TodoItemState},
    shared::database_error::{
        MyDBError, MyDBError::MongoDuplicateError, MyDBError::MongoQueryError,
        MyDBError::NotFoundError,
    },
};

#[allow(non_snake_case)]
pub async fn fetch_items(
    collection: &Collection<ItemDatabaseModel>,
    listId: &String,
    limit: i64,
    page: i64,
) -> Result<Vec<ItemDatabaseModel>, Box<dyn Error>> {
    let listId_as_object =
        ObjectId::from_str(&listId).map_err(|_| NotFoundError(listId.clone()))?;

    let filter = doc! { "listId": listId_as_object };

    let find_options = FindOptions::builder()
        .limit(limit)
        .skip(u64::try_from((page - 1) * limit).unwrap())
        .build();

    let mut cursor = collection
        .find(Some(filter), find_options)
        .await
        .map_err(MongoQueryError)?;

    let mut db_result: Vec<ItemDatabaseModel> = Vec::new();
    while let Some(doc) = cursor.next().await {
        match doc {
            Ok(item) => {
                println!("fetch_items returns {:?}", item);
                db_result.push(item);
            }
            Err(e) => {
                println!("Error processing document: {}", e);
                continue;
            }
        }
    }

    println!("fetch_items returns {:?}", db_result);

    Ok(db_result)
}

pub async fn create_item(
    collection: &Collection<ItemDatabaseModel>,
    item: &ItemDatabaseModel,
) -> Result<ItemDatabaseModel, Box<dyn Error>> {
    let result = match collection.insert_one(item, None).await {
        Ok(result) => result,
        Err(e) => {
            if e.to_string()
                .contains("E11000 duplicate key error collection")
            {
                return Err(Box::new(MongoDuplicateError(e)));
            }
            return Err(Box::new(MongoQueryError(e)));
        }
    };
    let inserted_id_string: String = result.inserted_id.as_object_id().unwrap().to_hex();
    let filter = doc! { "_id": result.inserted_id.as_object_id().unwrap() };
    let inserted_doc = match collection.find_one(filter, None).await {
        Ok(Some(doc)) => doc,
        Ok(None) => return Err(Box::new(NotFoundError(inserted_id_string.clone()))),
        Err(e) => return Err(Box::new(MongoQueryError(e))),
    };

    Ok(inserted_doc)
}
#[allow(non_snake_case)]
pub async fn get_single_item(
    collection: &Collection<ItemDatabaseModel>,
    listId: &String,
    id: &String,
) -> Result<ItemDatabaseModel, Box<dyn Error>> {
    let id_as_object = ObjectId::from_str(id).map_err(|_| NotFoundError(id.to_string()))?;
    let listId_as_object = ObjectId::from_str(listId).map_err(|_| NotFoundError(listId.clone()))?;

    let filter = doc! { "_id": id_as_object, "listId": listId_as_object};

    match collection.find_one(filter, None).await {
        Ok(Some(doc)) => Ok(doc),
        Ok(None) => Err(Box::new(NotFoundError(id.clone()))),
        Err(e) => Err(Box::new(MongoQueryError(e))),
    }
}
#[allow(non_snake_case)]
pub async fn edit_item(
    collection: &Collection<ItemDatabaseModel>,
    listId: &String,
    id: &String,
    item: &mut ItemDatabaseModel,
) -> Result<ItemDatabaseModel, Box<dyn Error>> {
    let id_as_object = ObjectId::from_str(id).map_err(|_| NotFoundError(id.to_string()))?;
    let listId_as_object = ObjectId::from_str(listId).map_err(|_| NotFoundError(listId.clone()))?;

    if item.state == TodoItemState::Done && item.completedDate.is_none() {
        let now = bson::DateTime::now();
        item.completedDate = Some(now);
    }

    // TBD: fix this - shouldn't have to remove createdDate from incoming doc
    let mut update_doc = bson::to_document(item).unwrap();
    update_doc.remove("createdDate");

    let filter = doc! { "_id": id_as_object, "listId": listId_as_object};
    let update = doc! { "$set": update_doc};

    let options = FindOneAndUpdateOptions::builder()
        .return_document(ReturnDocument::After)
        .build();

    match collection
        .find_one_and_update(filter, update, options)
        .await
    {
        Ok(Some(doc)) => Ok(doc),
        Ok(None) => Err(Box::new(NotFoundError(item._id.to_string()))),
        Err(e) => Err(Box::new(MyDBError::MongoQueryError(e))),
    }
}
#[allow(non_snake_case)]
pub async fn delete_item(
    collection: &Collection<ItemDatabaseModel>,
    listId: &String,
    id: &String,
) -> Result<ItemDatabaseModel, Box<dyn Error>> {
    println!("delete_item: listId: {:?}, id: {:?} ", listId, id);

    let id_as_object = ObjectId::from_str(id).map_err(|_| NotFoundError(id.to_string()))?;
    let listId_as_object = ObjectId::from_str(listId).map_err(|_| NotFoundError(listId.clone()))?;

    let filter = doc! { "_id": id_as_object, "listId": listId_as_object};

    match collection.find_one_and_delete(filter, None).await {
        Ok(Some(doc)) => Ok(doc),
        Ok(None) => Err(Box::new(NotFoundError(id.to_string()))),
        Err(e) => Err(Box::new(MyDBError::MongoQueryError(e))),
    }
}
/*

    // fn get_items_state_handler(&self,
    //     listid,
    //     state,
    //     skip, limit
    // ) -> Result {



/*
    const query = TodoItemModel.find({ listId: req.params.listId, state: req.params.state });
    const skip = req.query.skip ? parseInt(req.query.skip) : 0;
    const top = req.query.top ? parseInt(req.query.top) : 20;

    const lists = await query
        .skip(skip)
        .limit(top)
        .exec();

    res.json(lists);
*/

    // }
    // fn edit_list_items_state_handler() -> Result {
/*
    try {
        const completedDate = req.params.state === TodoItemState.Done ? new Date() : undefined;

        const updateTasks = req.body.map(
            id => TodoItemModel
                .findOneAndUpdate(
                    { listId: req.params.listId, _id: id },
                    { state: req.params.state, completedDate: completedDate })
                .orFail()
                .exec()
        );

        await Promise.all(updateTasks);

        res.status(204).send();
    }
    catch (err: any) {
        switch (err.constructor) {
        case mongoose.Error.CastError:
        case mongoose.Error.DocumentNotFoundError:
            return res.status(404).send();
        default:
            throw err;
        }
    }

*/
    //}
}
*/
