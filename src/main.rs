#[macro_use] extern crate rocket;
extern crate dotenvy;

use rocket::serde::{Serialize, json::Json};
use rocket::http::Header;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};

mod sql;
mod structs;

use structs::Recipe;
use sql::get_recipe_by_id;

// Allowing CORS
// Needs to be enabled to allow calls to the API
// Change the * in the Access-Controll-Allow-Origin line to the actual address of the frontend
// server for production
pub struct CORS;
#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info{
            name:"Adding CORS headers to response",
            kind: Kind::Response
        }
    }
    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}


#[get("/db/<id>")]
fn db(id:i64) -> Json<Recipe> {
    let out = get_recipe_by_id(id);
    Json(out)
}

#[launch]
fn rocket() -> _ {

    rocket::build()
        .mount("/", routes![db])
        .attach(CORS)
}
