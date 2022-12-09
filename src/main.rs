#[macro_use] extern crate rocket;
extern crate dotenvy;

use rocket::serde::json::Json;
use rocket::http::Header;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::State;

mod sql;
mod structs;

use structs::Recipe;
use sql::MongoRepo;

// mongodb://root:root@localhost:27017/?authMechanism=DEFAULT

#[get("/author/<author>")]
async fn author(author:&str, mdb: &State<MongoRepo>) -> Json<Recipe> {
    let mut out = Json(Recipe::empty());
    for i in mdb.get_by_author(author).await {
        out = Json(i);
    }
    out
}


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


#[launch]
async fn rocket() -> _ {
    let mdb = sql::MongoRepo::init();
    rocket::build()
        .mount("/", routes![author])
        .manage(mdb)
        .attach(CORS)
}
