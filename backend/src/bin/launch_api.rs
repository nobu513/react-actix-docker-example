// ref: https://actix.rs/

use actix_web::{get, web, App, HttpServer, HttpResponse, Responder};
use actix_cors::Cors;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
struct Student {
    name: String,
    age: i32,
}

#[get("/api/students")]
async fn get_students() -> HttpResponse {
    let result: Vec<Student> = vec![ 
        Student{
            name: String::from("hanako"),
            age: 22,
        },
        Student{
            name: String::from("nobu"),
            age: 33,
        }
    ];
    HttpResponse::Ok().json(result)
}  

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors: Cors = Cors::default()
            .allowed_origin("http://frontend:80")
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST"])
            .max_age(3600);
        App::new()
            .wrap(cors)
            .service(get_students)
    })
    .bind(("backend", 8080))?
    .run()
    .await
}