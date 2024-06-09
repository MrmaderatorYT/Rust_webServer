use actix_files::Files;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use tera::{Tera, Context};

async fn index(tera: web::Data<Tera>) -> impl Responder {
    let mut ctx = Context::new();
    ctx.insert("title", "CyberCraft Studio's Site");
    ctx.insert("content", "This is the home page.");

    match tera.render("index.html", &ctx) {
        Ok(rendered) => HttpResponse::Ok().content_type("text/html").body(rendered),
        Err(err) => {
            eprintln!("Template rendering error: {}", err);
            HttpResponse::InternalServerError().body("Internal Server Error")
        }
    }
}

async fn about(tera: web::Data<Tera>) -> impl Responder {
    let mut ctx = Context::new();
    ctx.insert("title", "About Us");
    ctx.insert("content", "This is the about page.");

    match tera.render("index.html", &ctx) {
        Ok(rendered) => HttpResponse::Ok().content_type("text/html").body(rendered),
        Err(err) => {
            eprintln!("Template rendering error: {}", err);
            HttpResponse::InternalServerError().body("Internal Server Error")
        }
    }
}

async fn contact() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("../templates/index.html"))
}

async fn content() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("../templates/index.html"))
}
async fn cybercraft_code() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("../templates/CyberCraftCode_list.html"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let tera = match Tera::new("templates/**/*") {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Parsing error(s): {}", e);
            std::process::exit(1);
        }
    };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(tera.clone()))
            .route("/", web::get().to(index))
            .route("/about", web::get().to(about))
            .route("/CyberCraft_Code", web::get().to(cybercraft_code))
            .route("/CyberCraft_Education", web::get().to(content))
            .route("/CyberCraft_News", web::get().to(content))
            .route("/contact", web::get().to(contact))
            .service(Files::new("/static", "static").show_files_listing())
            .service(Files::new("/CyberCraft_Code", "CyberCraft_Code").show_files_listing())
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
