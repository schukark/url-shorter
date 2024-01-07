use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use url_shorter::{add_entry, establish_connection, get_all_entries, get_entry, models::UrlEntry};

#[get("/api/all")]
async fn get_all() -> impl Responder {
    let connection = &mut establish_connection();
    let results: Vec<UrlEntry> = get_all_entries(connection);
    let mut together = String::new();

    for result in results {
        together = together + "\n" + &result.to_string();
    }

    HttpResponse::Ok().body(together)
}

#[get("/api/addr/{new_address}")]
async fn get_post(path: web::Path<String>) -> impl Responder {
    let short_url = path.into_inner();
    let short_url = String::from("sho.rt/") + &short_url;
    let connection = &mut establish_connection();

    let body = get_entry(connection, &short_url);

    if let Some(body) = body {
        HttpResponse::Ok().body(body.long_url)
    } else {
        HttpResponse::NotFound().body("There is no such url")
    }
}

#[post("/api/new/{new_addr}")]
async fn create_url(path: web::Path<String>) -> impl Responder {
    let long_url = path.into_inner();
    let connection = &mut establish_connection();
    let new_entry = add_entry(connection, &long_url);

    let answer = format!(
        "Added alias for {} -> {}",
        new_entry.long_url, new_entry.short_url
    );

    HttpResponse::Ok().body(answer)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .service(create_url)
            .service(get_post)
            .service(get_all)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
