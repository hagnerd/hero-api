#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use rocket_contrib::json::{Json, JsonValue};

mod hero;
use hero::{Hero};

#[get("/")]
fn read() -> JsonValue {
    json!(["hero1", "hero2"])
}

#[post("/", format="application/json", data="<hero>")]
fn create(hero: Json<Hero>) -> Json<Hero> {
    hero
}

#[put("/<id>", format="application/json", data="<hero>")]
fn update(id: i32, hero: Json<Hero>) -> Json<Hero> {
    println!("{}", id);

    hero
}

#[delete("/<id>")]
fn delete(id: i32) -> JsonValue {
    println!("{}", id);
    json!({"status": "ok"})
}

fn main() {
    rocket::ignite()
        .mount("/hero", routes![create, update, delete])
        .mount("/heroes", routes![read])
        .launch();
}
