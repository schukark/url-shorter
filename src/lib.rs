pub mod models;
pub mod schema;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

use self::models::{NewUrl, UrlEntry};

pub fn create_entry(conn: &mut PgConnection, short_url: &str, long_url: &str) -> UrlEntry {
    use crate::schema::urls;

    let new_post = NewUrl {
        short_url,
        long_url,
    };

    diesel::insert_into(urls::table)
        .values(&new_post)
        .returning(UrlEntry::as_returning())
        .get_result(conn)
        .expect("Error saving new post")
}

pub fn exists(conn: &mut PgConnection, query: &str) -> bool {
    use crate::schema::urls;
    use diesel::dsl::exists;
    use diesel::select;

    select(exists(urls::table.filter(urls::short_url.eq(query))))
        .get_result(conn)
        .unwrap()
}

pub fn get_entry(conn: &mut PgConnection, query: &str) -> Option<UrlEntry> {
    if !exists(conn, query) {
        return None;
    }

    use crate::schema::urls;

    let result = urls::table
        .filter(urls::short_url.eq(query))
        .get_result(conn)
        .unwrap();
    Some(result)
}

use rand::{distributions::Alphanumeric, Rng};

pub fn add_entry(connection: &mut PgConnection, long_url: &str) -> UrlEntry{

    let short_url;

    loop {
        let tmp: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(10)
            .map(char::from)
            .collect();

        let tmp = "sho.rt/".to_owned() + &tmp;

        if !exists(connection, &tmp) {
            short_url = tmp;
            break;
        }
    }

    create_entry(connection, &short_url, long_url)
}

pub fn get_all_entries(connection: &mut PgConnection) -> Vec<UrlEntry> {
    use crate::schema::urls;
    urls::table.load(connection).expect("Error loading posts")
}