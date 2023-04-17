mod models;
mod handler;
mod user_error;

use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{App, HttpServer, web};

mod schema;

use diesel::pg::PgConnection;
use diesel::{prelude::*, r2d2};
use diesel::r2d2::ConnectionManager;
use dotenvy::dotenv;
use std::env;


// Custom type for the connection pool
pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection() -> DbPool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    
    let manager = ConnectionManager::<PgConnection>::new(database_url.clone());

    // Establish a connection to the database
    let _connection = PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

    // Create a connection pool
    let pool: DbPool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    pool
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let pool = establish_connection();
    HttpServer::new(move || {
        App::new()

            .app_data(Data::new(pool.clone()))
            .wrap(Logger::default())
            .route("/", web::get().to(handler::health_checker))
            .route("/get", web::get().to(handler::get_users))
            .route("/add", web::post().to(handler::add_user))
            .route("/update/{id}", web::post().to(handler::update_user))
            .route("/delete/{id}", web::get().to(handler::delete_user))
            
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
