use actix_files as fs;
use actix_web::{get, web, App, HttpResponse, HttpServer};
use handlebars::Handlebars;
use motivations::MOTIVATIONS;
use rand::prelude::SliceRandom;
use rand::thread_rng;
use serde_json::json;
use std::io;

#[get("/")]
fn index(hb: web::Data<Handlebars>) -> HttpResponse {
    let mut rng = thread_rng();
    let &crab = [
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13",
    ]
    .choose(&mut rng)
    .unwrap();
    let &motivation = MOTIVATIONS.choose(&mut rng).unwrap();
    let data = json!({
        "motivation": motivation,
        "image": crab
    });
    let body = hb.render("motivation", &data).unwrap();

    HttpResponse::Ok().body(body)
}

fn main() -> io::Result<()> {
    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory(".html", "./templates")
        .unwrap();
    let handlebars_ref = web::Data::new(handlebars);

    HttpServer::new(move || {
        App::new()
            .register_data(handlebars_ref.clone())
            .service(index)
            .service(fs::Files::new("/public", "public/"))
    })
    .bind("127.0.0.1:8088")?
    .run()
}
