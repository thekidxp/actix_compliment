use actix_web::{App, HttpResponse, HttpServer, Responder, get, web};
use serde_json::json;
use std::io;
use handlebars::Handlebars;


#[get("/")]
fn index(hb: web::Data<Handlebars>) -> HttpResponse {
    let data = json!({
        "motivation": "hello"
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
    })
        .bind("127.0.0.1:8088")
        .unwrap()
        .run()
}
