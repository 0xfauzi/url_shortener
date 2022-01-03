#[macro_use]
extern crate rocket;

use dashmap::DashMap;

#[get("/")]
fn index() -> &'static str {
    "Hello world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(DashMap::<u32, String>::new()) // tell rocket to manage the state of this concurrent hashmap
        .mount("/", routes![index])
}