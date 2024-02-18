// Using the prelude module of the Cosmos crate makes easier to use the Rust Azure SDK for Cosmos DB.
use azure_data_cosmos::prelude::*;
//use azure_core::Context;
use serde::{Deserialize, Serialize};

// This is the stuct we want to use in our sample.
// Make sure to have a collection with partition key "number" for this example to
// work (you can create with this SDK too, check the examples folder for that task).
#[derive(Serialize, Deserialize, Debug)]
struct MySampleStruct {
    id: String,
    string: String,
    sample_type: String,
}

impl azure_data_cosmos::CosmosEntity for MySampleStruct {
    type Entity = String;

    fn partition_key(&self) -> Self::Entity {
        self.sample_type.clone()
    }
}

#[tokio::main]
async fn main() -> azure_core::Result<()> {

    dotenv::dotenv().ok();

    // Let's get Cosmos primary key and account name from env variables.
    let primary_key =
        std::env::var("COSMOS_PRIMARY_KEY").expect("Set env variable COSMOS_PRIMARY_KEY first!");
    let account = std::env::var("COSMOS_ACCOUNT").expect("Set env variable COSMOS_ACCOUNT first!");

    let database_name = std::env::args()
        .nth(1)
        .expect("please specify the database name as the first command line parameter");
    let collection_name = std::env::args()
        .nth(2)
        .expect("please specify the collection name as the second command line parameter");

    println!("database_name: {:?}", database_name);
    println!("collection_name: {:?}", collection_name);

    // First, create an authorization token. There are two types of tokens: primary and resource constrained.
    // Please check the Azure documentation or the examples folder on how to create and use token-based permissions.
    let authorization_token = AuthorizationToken::primary_key(&primary_key)?;

    // Next we will create a Cosmos client.
    let client = CosmosClient::new(account, authorization_token);

    // We know the database so we can obtain a database client.
    let database = client.database_client(database_name);
    // We know the collection so we can obtain a collection client.
    let collection = database.collection_client(collection_name);

    // Insert 10 documents
    println!("Inserting 10 documents...");
    for i in 1..10 {
        // define the document.
        let document_to_insert = MySampleStruct {
            id: format!("{}", i),
            string: "Something here".to_owned(),
            sample_type: if i % 2 == 0 { "red".to_string() } else { "blue".to_string() }
        };

        println!("document_to_insert: {:?}", document_to_insert);

        // insert it
        collection
            .create_document(document_to_insert)
            .is_upsert(true)
            .await?;

        println!("Document inserted!");
    }

    Ok(())
}