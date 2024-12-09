use actix_files as fs;
use actix_web::{get, web, App, HttpServer, HttpResponse, Responder, Error, error};
use serde::{Serialize, Deserialize};
use tera::Tera;

#[derive(Serialize, Deserialize, Clone)]
struct Book {
    title: String,
    author: String,
    cover: String
}

#[derive(Serialize)]
struct RandObject {
    value: i32
}

fn get_books() -> Vec<Book> {
    return [
        Book{
            title: "Scrum: The Art of Doing Twice the Work in Half the Time".to_string(),
            author: "Jeff Sutherland".to_string(),
            cover: "scrum.jpg".to_string()
        },
        Book{
            title: "The Lean Startup: How Constant Innovation Creates Radically Successful Businesses".to_string(),
            author: "Eric Ries".to_string(),
            cover: "lean.jpg".to_string(),
        },
        Book{
            title: "Crossing the Chasm".to_string(),
            author: "Geoffrey A. Moore".to_string(),
            cover: "chasm.jpg".to_string(),
        },
        Book{
            title: "The Pragmatic Programmer: From Journeyman to Master".to_string(),
            author: "David Thomas".to_string(),
            cover: "pragmatic.jpg".to_string(),
        },
        Book{
            title: "The Mythical Man-Month: Essays on Software Engineering".to_string(),
            author: "Frederick P. Brooks Jr.".to_string(),
            cover: "month.jpg".to_string(),
        },
        Book{
            title: "Don't Make Me Think, Revisited: A Common Sense Approach to Web Usability".to_string(),
            author: "Steve Krug".to_string(),
            cover: "think.jpg".to_string(),
        }
    ].to_vec();
}

#[get("/")]
async fn index(tmpl: web::Data<Tera>) -> Result<HttpResponse, Error> {

    let mut ctx = tera::Context::new();
    let books = get_books();
    ctx.insert("books", &books);
    let view =
        tmpl.render("index.html.tera", &ctx)
            .map_err(|e| error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(view))
}

#[get("/rand")]
async fn random() -> Result<impl Responder, Error> {
    let obj = RandObject {
        value: fastrand::i32(..)
    };
    Ok(web::Json(obj))
}


#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server at http://localhost:8080");
    HttpServer::new(|| {
        let templates = Tera::new("templates/**/*").unwrap();
        App::new()
            .app_data(web::Data::new(templates))
            .service(fs::Files::new("/static", "static"))
            .service(index)
            .service(random)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
