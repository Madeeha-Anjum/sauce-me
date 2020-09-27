#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::{response::Redirect, Request, State};
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Deserialize)]
struct Data {
    url: String,
}

#[derive(Serialize)]
struct Response {
    url: String,
}

#[derive(Serialize, Deserialize)]
struct myState {
    map: HashMap<String, String>,
    curr_num: i32,
}

impl myState {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
            curr_num: 0,
        }
    }
}

#[get("/")]
fn index() -> &'static str {
    "It works!"
}

#[post("/shorten", data = "<input>")]
fn shorten(input: Json<Data>, state: State<Mutex<myState>>) -> Json<HashMap<String, String>> {
    let mut state = state.lock().unwrap();

    state.curr_num += 1;
    let curr_num = state.curr_num.to_string();
    state.map.insert(curr_num, input.0.url);

    Json(state.map.clone())
}

#[get("/redirect/<code>")]
fn redirect(code: String) -> Redirect {
    println!("Code: {}", code);
    Redirect::to("https://google.com/")
}

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Sorry, '{}' is not a valid path.", req.uri())
}

fn main() {
    rocket::ignite()
        .manage(Mutex::new(myState::new()))
        .mount("/api/v1", routes![index, shorten, redirect])
        .register(catchers![not_found])
        .launch();
}
