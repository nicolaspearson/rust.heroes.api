#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;
extern crate dotenv;

use rocket_contrib::json::{Json, JsonValue};

pub mod db;
pub mod hero;
pub mod schema;

use hero::Hero;

#[get("/?<id>")]
fn get_hero(id: i32, connection: db::Connection) -> Json<JsonValue> {
	Json(json!(Hero::get(id, &connection)))
}

#[get("/")]
fn get_heroes(connection: db::Connection) -> Json<JsonValue> {
	Json(json!(Hero::get_all(&connection)))
}

#[post("/", data = "<hero>")]
fn create_hero(hero: Json<Hero>, connection: db::Connection) -> Json<Hero> {
	let insert = Hero {
		id: None,
		..hero.into_inner()
	};
	Json(Hero::create(insert, &connection))
}

#[put("/<id>", data = "<hero>")]
fn update_hero(id: i32, hero: Json<Hero>, connection: db::Connection) -> Json<JsonValue> {
	let update = Hero {
		id: Some(id),
		..hero.into_inner()
	};
	Json(json!({ "success": Hero::update(id, update, &connection) }))
}

#[delete("/<id>")]
fn delete_hero(id: i32, connection: db::Connection) -> Json<JsonValue> {
	Json(json!({ "success": Hero::delete(id, &connection) }))
}

fn main() {
	dotenv::dotenv().expect("Failed to read .env file");

	rocket::ignite()
		.mount(
			"/hero",
			routes![get_hero, create_hero, update_hero, delete_hero],
		)
		.mount("/heroes", routes![get_heroes])
		.manage(db::connect())
		.launch();
}
