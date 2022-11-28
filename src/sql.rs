use sqlite;

use crate::structs::Recipe;

pub fn get_recipe_by_id(id:i64) -> Recipe{
    let connection = sqlite::open("test_db.db").expect("Failed to connect to db");
    let query = "SELECT * FROM RECIPE WHERE id = ?";
    
    let mut statement = connection.prepare(query).unwrap();
    statement.bind((1, id)).unwrap();

    let mut out = Recipe {
        id: 0,
        title: "FAILED TO READ".to_string(),
        author: "Mindcool24".to_string(),
        ingredients: "null".to_string(),
        instructions: "null".to_string(),
        notes: "Please report that there was an error to mindcool24@jsociety.xyz".to_string(),
    };

    while let Ok(sqlite::State::Row) = statement.next() {
        out = Recipe {
            id: statement.read::<i64, _>("id").unwrap(),
            title: statement.read::<String, _>("Title").unwrap(),
            author: statement.read::<String, _>("Author").unwrap(),
            ingredients: statement.read::<String, _>("Ingredients").unwrap(),
            instructions: statement.read::<String, _>("Instructions").unwrap(),
            notes: statement.read::<String, _>("Notes").unwrap(),
            };
       }
    out
}
