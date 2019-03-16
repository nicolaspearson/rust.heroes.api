use super::schema::hero;
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;

use chrono::NaiveDateTime;

#[table_name = "hero"]
#[derive(AsChangeset, Serialize, Deserialize, Queryable, Insertable)]
pub struct Hero {
	pub id: Option<i32>,
	pub name: String,
	pub identity: String,
	pub hometown: String,
	pub age: i32,
	pub created_at: Option<NaiveDateTime>,
	pub updated_at: Option<NaiveDateTime>,
	pub deleted_at: Option<NaiveDateTime>,
}

impl Hero {
	pub fn get(id: i32, connection: &PgConnection) -> Hero {
		return hero::table.find(id).first(connection).unwrap();
	}

	pub fn get_all(connection: &PgConnection) -> Vec<Hero> {
		hero::table
			.order(hero::id)
			.load::<Hero>(connection)
			.unwrap()
	}

	pub fn create(hero_item: Hero, connection: &PgConnection) -> Hero {
		diesel::insert_into(hero::table)
			.values(&hero_item)
			.execute(connection)
			.expect("Error creating new hero");

		return hero::table
			.order(hero::id.desc())
			.first(connection)
			.unwrap();
	}

	pub fn update(id: i32, hero_item: Hero, connection: &PgConnection) -> bool {
		diesel::update(hero::table.find(id))
			.set(&hero_item)
			.execute(connection)
			.is_ok()
	}

	pub fn delete(id: i32, connection: &PgConnection) -> bool {
		diesel::delete(hero::table.find(id))
			.execute(connection)
			.is_ok()
	}
}
