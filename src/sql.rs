use mongodb::{bson::doc, options::{FindOptions, FindOneOptions}};
use mongodb::sync::{Client, Collection};
use mongodb::bson::oid::ObjectId;

use crate::structs::Recipe;
use crate::structs::NewRecipe;

pub struct MongoRepo {
    col: Collection<Recipe>,
    sub_col: Collection<NewRecipe>
}

impl MongoRepo {
    pub fn init() -> Self {
        let uri = "mongodb://root:root@localhost:27017";
        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database("recipe_book");
        let col: Collection<Recipe> = db.collection("recipes");
        let sub_col: Collection<NewRecipe> = db.collection("recipes");
        MongoRepo{col, sub_col}
    }

    pub async fn get_by_author(&self, author: &str) -> Vec<Recipe> {
        // Setting up filter and cursor
        let filter = doc!{"author":author};
        let find_options = FindOptions::builder().sort(doc!{"title":1}).build();
        let cursor = self.col.find(filter, find_options).expect("Failed to make cursor");

        let mut out: Vec<Recipe> = Vec::new();

        // Getting results
        for recipe in cursor {
            out.push(recipe.expect("Failed to grap recipe"));
        }
        out
    }
    
    pub async fn get_by_id(&self, id: ObjectId) -> Recipe{
        let filter = doc!{"_id": id};
        let find_options = FindOneOptions::builder().build();
        let cursor = self.col.find_one(filter, find_options).expect("Failed to make cursor");

        let mut out: Vec<Recipe> = Vec::new();
        
        cursor.expect("Failed to grab recipe")
    }

    pub async fn add_recipe(&self, recipe: NewRecipe) {
        self.sub_col.insert_one(recipe, None).expect("Failed to insert");
    }

}
