use diesel;
use diesel::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct Hero {
    pub id: Option<i32>,
    pub name: String,
    pub identity: String,
    pub hometown: String,
    pub age: i32,
}

impl Hero {
    pub fn new() -> Hero {
        Hero {
            id: Some(1),
            name: String::from("Batman"),
            identity: String::from("Bruce Wayne"),
            hometown: String::from("Gotham"),
            age: 32,
        }
    }
}
