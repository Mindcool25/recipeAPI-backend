use rocket::serde::{Serialize, Deserialize};

use std::borrow::Borrow;

use mongodb::bson::oid::ObjectId;

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Recipe {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,
    pub title: String,
    pub author: String,
    pub ingredients: Vec<Ingredient>,
    pub instructions: Vec<String>,
    pub notes: String,
}

impl Recipe {
    pub fn empty() -> Self {
        Recipe {
            _id: None,
            title: "".to_string(),
            author: "".to_string(),
            ingredients: vec![],
            instructions: vec![],
            notes: "".to_string()
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct NewRecipe {
    pub title: String,
    pub author: String,
    pub ingredients: Vec<Ingredient>,
    pub instructions: Vec<String>,
    pub notes: String,
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Ingredient {
    pub i_name: String,
    pub amount: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Instruction {
    pub instruction: String
}
