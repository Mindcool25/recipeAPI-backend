use mongodb::{bson::doc, options::{FindOptions, ClientOptions}};
use mongodb::sync::{Client, Collection};


use crate::structs::Recipe;

pub struct MongoRepo {
    col: Collection<Recipe>
}

impl MongoRepo {
    pub fn init() -> Self {
        let uri = "mongodb://root:root@localhost:27017";
        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database("recipe_book");
        let col: Collection<Recipe> = db.collection("recipes");
        MongoRepo{col}
    }

    pub async fn get_by_author(&self, author: &str) -> Vec<Recipe> {
        // Setting up filter and cursor
        let filter = doc!{"author":author};
        let find_options = FindOptions::builder().sort(doc!{"title":1}).build();
        let mut cursor = self.col.find(filter, find_options).expect("Failed to make cursor");

        let mut out: Vec<Recipe> = Vec::new();

        // Getting results
        while let Some(recipe) = cursor.next() {
            out.push(recipe.expect("Failed to grab recipe"));
        }
        out
    }

}


pub async fn get() -> String {
    // Set options for DB
    let mut client_options = ClientOptions::parse("mongodb://root:root@localhost:27017").expect("Failed to set options");
    client_options.app_name = Some("Recipes".to_string());

    // Connect to DB with options
    let client = Client::with_options(client_options).expect("Failed to connect");

    // Get to the recipe database and collection
    let r_db = client.database("recipe_book");
    let col = r_db.collection::<Recipe>("recipes");

    // Set up filter and options to get the correct stuff
    let filter = doc!{"author":"Mindcool24"};
    let find_options = FindOptions::builder().sort(doc!{"title":1}).build();
    let mut cursor = col.find(filter, find_options).expect("Failed to make cursor");
    while let Some(recipe) = cursor.next(){
        println!("{:?}", recipe.expect("Hm"));
    }
    "It works".to_string()
}
