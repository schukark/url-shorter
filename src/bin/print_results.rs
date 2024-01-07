use self::models::*;
use diesel::prelude::*;
use url_shorter::*;

fn main() {
    use self::schema::urls::dsl::*;

    let connection = &mut establish_connection();
    let results: Vec<UrlEntry> = urls.load(connection).expect("Error loading posts");

    println!("Displaying {} urls", results.len());
    for entry in results {
        println!("{}", entry.short_url);
        println!("{}", entry.long_url);
    }
}
