#[macro_use]
extern crate rocket;

use dashmap::DashMap;
use rand::{Rng, thread_rng};
use rocket::response::status::BadRequest;
use rocket::State;

#[get("/")]
fn index() -> &'static str {
    "Hello world!"
}

#[post("/api/shorten?<url>")]
fn shorten(url: String, state: &State<DashMap<u32, String>>) -> Result<String, BadRequest<&str>> {
    if url.is_empty() {
        Err(BadRequest(Some("URL is empty!")))
    } else {
        let key: u32 = thread_rng.gen();
        state.insert(key, url);
        Ok(key.to_string())
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(DashMap::<u32, String>::new()) // tell rocket to manage the state of this concurrent hashmap
        .mount("/", routes![index])
}