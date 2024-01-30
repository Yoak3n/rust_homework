use std::env;

use auth_server::{models::query:: QueryModel, state::AppState};

use actix_web::{get, web,App, HttpServer,Responder,HttpResponse};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;


#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}
use auth_server::controller::*;
use chrono::NaiveDateTime;
#[get("/callback")]
async fn callback(
    info: web::Query<QueryModel>,
    share_data: web::Data<AppState>
) ->  HttpResponse{
    let query = QueryModel{
        code: info.code.to_string(),
        state: info.state.to_string(),
        time: Some(NaiveDateTime::from(chrono::Utc::now().naive_local()))
    };
    let info = new_data(&share_data.db,query).await;
    HttpResponse::Ok().json(info)
}

#[get("/auth/{state}")]
async fn auth(
    info: web::Path<String>,
    share_data: web::Data<AppState>
) ->  HttpResponse{
    let r = get_data(&share_data.db,info.as_str()).await;
    
    HttpResponse::Ok().json(r)
}

#[tokio::main] // #[actix_web::main] 
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
    .expect("DATABASE_URL NOT FOUND");

    let db_pool = PgPoolOptions::new()
    .connect(&database_url)
    .await
    .unwrap();

    let share_data = web::Data::new(AppState{
        db: db_pool
    });

    HttpServer::new(move|| {
        App::new()
        .app_data(share_data.clone())
        .service(greet)
        .service(callback)
        .service(auth)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}