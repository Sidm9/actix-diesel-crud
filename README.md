## Introduction

Rust is a powerful, modern, systems programming language that has been gaining popularity in recent years due to its focus on safety, performance, and concurrency. Rust's unique features such as fearless concurrency, zero-cost abstractions, and memory safety without garbage collection make it an ideal choice for building fast and reliable applications. Its expressive syntax and robust type system enable developers to write clean, maintainable code with minimal runtime overhead.

In this article, we will explore how Rust can be utilized as a language for creating API endpoints using the Actix web framework. By implementing a simple CRUD application, we will dive into various aspects of Rust and Actix, from adding dependencies to handling errors. The aim is to help you understand the power of Rust as an API language and provide you with a practical example that you can use as a foundation for your projects.

Actix-web is a high-performance web framework for Rust, designed to be flexible and easy to use. It is built on top of the Actix actor framework, which was inspired by the Erlang/OTP actor model and provides Rust with a powerful and efficient concurrency model. Actix-web leverages Rust's async/await features, enabling it to handle thousands of concurrent connections while maintaining low latency and efficient resource usage. Some of the key features of Actix-web include a modular design, powerful routing, middlewares, and plug-and-play support for various data formats and protocols.

Before we dive into building our CRUD application, let's discuss the prerequisites. To follow along with this tutorial, you should have Rust 1.59 or later installed on your system, as well as the PostgreSQL database server. If you're missing any of these components, please refer to their respective documentation for installation instructions.

To kick off our project, let's create a new Rust application by running the following command:

```bash
cargo init rust_crud
```  

This will generate a new Rust project called "rust_crud" with a **```cargo.toml```** file. 

Here are the dependencies for our Rust project, as specified in the **```cargo.toml```** file. We will explain each of these dependencies in detail as we go through the tutorial.

```toml
name = "rust_crud"
version = "0.1.0"
edition = "2021"

[dependencies]
actix-web = "4.3.1"
actix-rt = "2.8.0"
chrono = { version = "0.4.24", features = ["serde"] }
serde = { version = "1.0.160", features = ["derive"] }
uuid = { version = "1.3.1", features = ["serde" , "v4"] }
diesel = { version = "2.0.3", features = ["postgres" , "uuid" , "r2d2" , "chrono"] }
dotenvy = "0.15"
```

Keep in mind that using different versions of these dependencies may cause compatibility issues or errors in the code, so it's important to use the specified versions.

To set up the database and manage its configuration, we will use environment variables. This will allow us to easily switch between different environments and configurations without modifying our code. To set up the environment variable for our PostgreSQL database, we can run the following command: 

```bash
echo DATABASE_URL=postgres://postgres@localhost/rust_crud > .env
```

This will create a new ```.env``` file in our project directory with the ```DATABASE_URL``` variable set to our PostgreSQL database's URL. 

## Structure of application

Our application will have 4 files. ```main.rs```, ```handler.rs```, ```user_error.rs``` and ```models.rs```.

### main.rs

The ```main.rs``` file is the entry point of our application. It initializes the Actix-web server, sets up the necessary routes, and starts listening to incoming requests.

We use the ```actix_web``` crate to create the server and define the routes. We also use `diesel` to interact with the PostgreSQL database.

### handler.rs

The ```handler.rs``` file defines the request handlers for our CRUD operations. It contains functions to handle requests for getting all users, getting a user by ID, adding a new user, updating a user, and deleting a user.

Each handler function takes an ```HttpRequest``` argument, which contains information about the incoming request, and returns an ```HttpResponse``` or a custom error response defined in ```user_error.rs```.

### models.rs

We will use models.rs to define all our models for the users table and data structures for the application.

### user_error.rs

The ```user_error``` file will be responsible for error handling and our custom implementation of responses.

So create 3 files handler.rs , models.rs and user_error.rs for the same inside the src folder.

*NIX systems

```bash
touch src/handler.rs
touch src/models.rs
touch src/user_error.rs
```

Windows

```bash
type nul > src/handler.rs
type nul > src/models.rs
type nul > src/user_error.rs
```

Now that we have our dependencies and files in place, let's start by setting up the basic structure of our application. This will include importing the required crates, initializing the Actix-web server, setting handlers and configuring the routes for our CRUD application.

## Application Routes

Our application will have the following routes:

```/:``` health check route (for testing purposes) [GET]
```/get:``` get all users [GET]
```/add:``` add a new user [POST]
```/get/{id}:``` get a user by ID [GET]
```/update/{id}:``` update a user [POST]
```/delete/{id}:``` delete a user [GET]


```rust
// main.rs
mod handler;
// Import the required crates
use actix_web::{web, App, HttpServer};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // Configure and start the Actix-web server
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(handler::health_checker))
            .route("/get", web::get().to(handler::get_users))
            .route("/add", web::post().to(handler::add_user))
            .route("/get/{id}" , web::get().to(handler::get_user_by_id))
            .route("/update/{id}", web::post().to(handler::update_user))
            .route("/delete/{id}", web::get().to(handler::delete_user)) 
    })
    .bind("127.0.0.1:8080")? // Bind the server to the specified address and port
    .run() // Run the server
    .await
}
```

In the code above, we start by importing the necessary crates and define some simple functions. We then define the **```main```** function, which initializes and starts the Actix-web server, binds it to a specific address and port (in this case, "127.0.0.1:8080"), and sets up the route for the different paths.

In the handler.rs file

```rust
// handler.rs

use actix_web::{HttpResponse, Responder};

pub async fn health_checker() -> impl Responder {
    HttpResponse::Ok().body("Working!")
}

pub async fn get_users() -> impl Responder {
    HttpResponse::Ok().json("Get all users")
}

pub async fn get_user_by_id() -> impl Responder {
    HttpResponse::Ok().json("Get user by id")
}

pub async fn add_user() -> impl Responder {
    HttpResponse::Ok().json("Add user")
}

pub async fn update_user() -> impl Responder {
    HttpResponse::Ok().json("Update user")
}

pub async fn delete_user() -> impl Responder {
    HttpResponse::Ok().json("delete user")
}


```
In the handler.rs file notice we have a return type of ```impl Responder``` . This is because we are using the ```actix-web``` crate, which provides a ```Responder``` trait that allows us to easily convert our response into a ```HttpResponse```.


We can now start the server with the following command:

```bash
cargo run
```
But we will be updating the code often so we would like to make it run every time we save the changes. kinda like hot reload. we can achieve this by `cargo watch` command. We can install it using the following command. 

```bash
cargo install cargo-watch
```

Once `cargo watch` is installed, we can run our application with ```cargo watch -x run``` .Our application should compile correctly and listen to the port 8080.
You can close the server for now by pressing ```Ctrl + C``` or ```Command + C```. Or continue to the next step.

At this point, we have a basic Actix-web server up and running. However, it doesn't interact with the database or handle any CRUD operations yet. 

## **Setting up Diesel**

Now that our basic Actix-web server is up and running, it's time to introduce Diesel into our application. Diesel is an extensible ORM and query builder for Rust that focuses on type safety and ease of use. It provides a clean and efficient way to interact with databases, allowing you to write expressive and type-safe queries with minimal boilerplate.

To get started with Diesel, we need to install the Diesel CLI tool, which will help us manage database-related tasks, such as setting up the database, running migrations, and generating the schema. Install the Diesel CLI tool using the following command:

```bash
cargo install diesel_cli --no-default-features --features postgres
```

This command installs the Diesel CLI with PostgreSQL support. Make sure your PostgreSQL server is running, and you have the correct ```DATABASE_URL``` in the ```.env``` file, as mentioned earlier.

Next, we need to set up the database using Diesel CLI:

```bash
diesel setup
```

This command will create a new directory named ```migrations``` in our project folder, which will be used to manage database migrations. Now, let's create a migration to create the `users` table:

```bash
diesel migration generate create_users
```

This will generate two new files in the ```migrations``` directory, named ```up.sql``` and ```down.sql```. Open the ```up.sql``` file and add the following SQL code to create the ```users``` table:

```sql
-- up.sql
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    user_id UUID NOT NULL UNIQUE,
    first_name VARCHAR NOT NULL,
    last_name VARCHAR NOT NULL,
    email VARCHAR NOT NULL UNIQUE,
    created_at TIMESTAMP NOT NULL
);
```

In the ```down.sql``` file, add the following SQL code to drop the ```users``` table:

```sql
-- down.sql
DROP TABLE users;
```

Now, run the migration to apply these changes to the database:

```bash
diesel migration run
```

After running the migration, Diesel CLI will generate a ```schema.rs``` file in the ```src``` directory. This file contains the schema for our ```users``` table, which will be used to interact with the database using Diesel. The ```schema.rs``` file should look like this:

```rust
// src/schema.rs
diesel::table! {
    users (id) {
        id -> Int4,
        user_id -> Uuid,
        first_name -> Varchar,
        last_name -> Varchar,
        email -> Varchar,
        created_at -> Timestamp,
    }
}
```

Now we have set up Diesel and our ```users``` schema. In the next steps, we will start building our CRUD API by creating the necessary database models, routes, and handlers using Diesel and Actix-web.

Now that we have set up our database schema, let's create the Rust data structures that represent our users in the application. To do this, we will define several structs in a new ```models.rs``` file:

in the models.rs file we need these imports

```rust
// src/models.rs
use chrono::NaiveDateTime;
use diesel::prelude::*;
use crate::schema::users;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
```
    
```chrono::NaiveDateTime```: This struct represents a date and time without a time zone. We will use this to store the creation timestamp of our users.

```diesel::prelude::*```: This module contains the most commonly used traits and types in Diesel. We will use this to interact with the database.

```schema::users```: This module contains the schema for the ```users``` table. We will use this to interact with the database.

```serde::{Deserialize, Serialize}```: This module contains the ```Deserialize``` and ```Serialize``` traits, which we will use to convert our structs into JSON.

```uuid::Uuid```: This struct represents a universally unique identifier (UUID). We will use this to generate a unique ID for each user.

```rust
#[derive(Serialize)]
pub struct GenericResponse<T> {
    pub status: String,
    pub message: String,
    pub data: Option<T>,
}
```
```GenericResponse<T>```: This struct represents a generic API response format that includes a status, message, and optional data. We can use this to provide a consistent response structure for our API endpoints.

```rust
// src/models.rs
#[derive(Debug, Serialize, Deserialize, Insertable, Queryable)]
#[diesel(table_name = users)]
pub struct Users {
    pub id: Option<i32>,
    pub user_id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub created_at: NaiveDateTime,
}
```
```Users```: This struct represents the user object as it is stored in the database. It includes fields for the user's ID, UUID, first name, last name, email, and creation timestamp.

```rust
// src/models.rs
#[derive(Queryable, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct User {
    pub id: i32,
    pub user_id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub created_at: NaiveDateTime,
}
```

```User```: This struct is a simplified representation of the ```Users``` struct without the ```Option<i32>``` type for the `id` field. We can use this when querying the database to retrieve user data.

```rust
// src/models.rs

#[derive(Insertable, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}
```

```NewUser```: This struct represents a new user object to be inserted into the database. It includes fields for the user's first name, last name, and email.

```rust
// src/models.rs
#[derive(AsChangeset, Debug, Deserialize)]
#[diesel(table_name = users)]
pub struct UpdateUser {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
}
```

```UpdateUser```: This struct represents an update operation on a user object. It includes optional fields for the user's first name, last name, and email. When updating a user, we can set only the fields we want to update and leave the rest as ```None```.

These structs serve as the foundation for our CRUD operations. In the next steps, we will create the necessary routes, handlers, and database functions to interact with the `users` table using Diesel and Actix-web.

Now that we have our data structures in place, let's handle errors that may occur during our CRUD operations. For this, we will create a new `user_error.rs` file that contains a custom ```UserError``` enum to represent different types of errors related to users.

Add the following code to the ```user_error.rs``` file:

```rust
// src/user_error.rs

use std::fmt;
use actix_web::{HttpResponse, ResponseError};
use diesel::result::Error as DieselError;

// Define a custom error type for user operations.
#[derive(Debug)]
pub enum UserError {
    NotFound,
    AddingUser,
    UpdatingUser,
    DeletingUser,
    DieselError(DieselError),
}
// Implement the `Display` trait for the `UserError` enum.
impl fmt::Display for UserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UserError::NotFound => write!(f, "User not found"),
            UserError::AddingUser => write!(f, "Error adding user"),
            UserError::UpdatingUser => write!(f, "Error updating user"),
            UserError::DeletingUser => write!(f, "Error deleting user"),
            UserError::DieselError(diesel_error) => write!(f, "Diesel error: {}", diesel_error),
        }
    }
}

impl ResponseError for UserError {
    fn error_response(&self) -> HttpResponse {
        match self {
            UserError::NotFound => HttpResponse::NotFound().json(self.to_string()),
            _ => HttpResponse::InternalServerError().json(self.to_string()),
        }
    }
}

```

The ```UserError``` enum has variants for different types of user-related errors, such as not found, adding, updating, or deleting a user, and a variant for Diesel errors.

We implement the ```fmt::Display``` trait for the ```UserError``` enum, allowing us to convert the error variants to human-readable error messages.

To handle the error responses in our Actix-web application, we implement the `ResponseError` trait for the ```UserError``` enum. The ```error_response``` method converts the ```UserError``` variants to appropriate HTTP responses. For example, if the error is a `NotFound` variant, the method returns a ```NotFound``` HTTP response. For all other error variants, it returns an ```InternalServerError``` HTTP response.

With this custom error handling in place, we can use the ```UserError``` enum throughout our application to provide meaningful error messages and appropriate HTTP responses for different types of user-related errors.

Now that we have our data structures, handlers, and error handling in place, it's time to update the ```main.rs``` file to incorporate Diesel and set up the database connection. Add the following imports and the custom ```DbPool``` type:

```rust
// src/main.rs

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

 // #[actix_rt::main]
 // async fn main() ... 

```

The ```DbPool``` type is a custom type alias for a connection pool using the ```r2d2::Pool``` struct with a ```ConnectionManager<PgConnection>```. Connection pools are essential to manage the database connections and efficiently reuse them in a concurrent environment like a web application.

Next, add the ```establish_connection()``` function to create the connection pool:

```rust

// src/main.rs

// ... pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

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

    // #[actix_rt::main]
    // async fn main() ...

```

Since Diesel does not support asynchronous operations, we have to use a combination of Actix-rt and the r2d2 connection pool to manage concurrent requests to the database efficiently. The ```actix_rt::main``` attribute macro will run our application using Actix's runtime system.

Now, update the ```main``` function to use the connection pool:

```rust
// src/main.rs

// pub fn establish_connection() ...

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
            .route("/get/{id}" , web::get().to(handler::get_user_by_id))
            .route("/update/{id}", web::post().to(handler::update_user))
            .route("/delete/{id}", web::get().to(handler::delete_user))

    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
```

The ```establish_connection()``` function is called to create the connection pool. The connection pool is then added to the application data using ```Data::new(pool.clone())```. This allows us to access the connection pool in our request handlers and perform database operations.

With these updates, our application is now using Diesel for database operations and efficiently managing connections using Actix-rt and r2d2.

Back in our [handler.rs](http://handler.rs) file we can implement the functionality of each route.

```rust
// handler.rs

use crate::{models, user_error::UserError, DbPool};
use actix_web::{web, HttpResponse, Responder};
use chrono::prelude::*;
use diesel::prelude::*;
use uuid::Uuid;

fn get_conn_from_db(
    pool: web::Data<diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<PgConnection>>>,
) -> diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<PgConnection>> {
    let conn = pool
        .get()
        .expect("Error getting a connection from the pool");
    conn
}

// pub async fn health_checker() ...

```

The ```get_conn_from_db``` is a helper function takes a ```Pool``` object and returns a `PooledConnection` object. It is used to get a database connection from the connection pool in order to perform database operations. 

```rust

// hanlder.rs
pub async fn health_checker() -> impl Responder {

    let response = models::GenericResponse::<()> {
        status: "OK".to_string(),
        message: "Working".to_string(),
        data: None,
    };
    HttpResponse::Ok().json(response)
}

pub async fn get_users(pool: web::Data<DbPool>) -> Result<HttpResponse, UserError> {
    let user_result = web::block(move || {
        let mut conn = get_conn_from_db(pool);

        use crate::schema::users::dsl::*;

        let user = users.load::<models::User>(&mut conn);

        user
    })
    .await
    .map_err(|_| UserError::NotFound)?;

    match user_result {
        Ok(users_list) => Ok(HttpResponse::Ok().json(models::GenericResponse {
            status: "OK".to_string(),
            message: "Users Fetched successfully".to_string(),
            data: Some(users_list),
        })),
        Err(diesel_error) => Err(UserError::from(UserError::DieselError(diesel_error))),
    }
}
```

Here's an overview of the **```get_users```** function's process:

1  Obtain a connection to the database by calling **```get_conn_from_db```** and passing the connection pool.

2 Import the users table DSL (Domain Specific Language) from the schema.

3 Use Diesel to load the users from the database into a Rust data structure of type **```models::User```**.

4 The Diesel operation is performed inside the **```web::block```** function, which moves the synchronous database query to a separate thread to avoid blocking the async event loop. If an error occurs, it is mapped to a **```UserError::NotFound```** error.

5 Finally, the function checks the result of the database operation. This uses our ```user_error.rs``` UserError enum for error handling. 

The rest of the functions are similar.

```rust
// handler.rs

// pub async fn add_user() ...

// Get user by id
pub async fn get_user_by_id(pool: web::Data<DbPool> , path: web::Path<(String,)>) -> impl actix_web::Responder {

    let user_result = web::block(move || {
        let parsed_user_id = Uuid::parse_str(&path.into_inner().0).expect("Error parsing user_id");

        let mut conn = get_conn_from_db(pool);

        use crate::schema::users::dsl::*;

        let user = users
            .filter(user_id.eq(parsed_user_id))
            .load::<models::User>(&mut conn);

        user
    })

    .await
    .map_err(|_| UserError::NotFound)?;

    match user_result {
        Ok(users_list) => Ok(HttpResponse::Ok().json(models::GenericResponse {
            status: "OK".to_string(),
            message: "User Fetched successfully".to_string(),
            data: Some(users_list),
        })),
        Err(diesel_error) => Err(UserError::from(UserError::DieselError(diesel_error))),
    }
}

// Add user
pub async fn add_user(
    pool: web::Data<DbPool>,
    form: web::Json<models::NewUser>,
) -> Result<HttpResponse, UserError> {
    let user_result = web::block(move || {
        let mut conn = get_conn_from_db(pool);

        use crate::schema::users::dsl::*;

        let new_user = models::Users {
            id: None,
            user_id: Uuid::new_v4(),
            first_name: form.first_name.to_string(),
            last_name: form.last_name.to_string(),
            email: form.email.to_string(),
            created_at: Local::now().naive_local(),
        };

        diesel::insert_into(users)
            .values(&new_user)
            .execute(&mut conn)?;

        users
            .order(id.desc())
            .limit(1)
            .load::<models::User>(&mut conn)
    })
    .await
    .map_err(|_| UserError::AddingUser)?;

    match user_result {
        Ok(users_list) => Ok(HttpResponse::Ok().json(models::GenericResponse {
            status: "OK".to_string(),
            message: "User added successfully".to_string(),
            data: Some(users_list),
        })),
        Err(diesel_error) => Err(UserError::from(UserError::DieselError(diesel_error))),
    }
}

// Update user
pub async fn update_user(
    pool: web::Data<DbPool>,
    path: web::Path<(String,)>,
    form: web::Json<models::UpdateUser>,
) -> impl actix_web::Responder {
    let user_result = web::block(move || {
        let parsed_user_id = Uuid::parse_str(&path.into_inner().0).expect("Error parsing user_id");

        let mut conn = get_conn_from_db(pool);

        use crate::schema::users::dsl::*;

        let updated_user = models::UpdateUser {
            first_name: Some(form.first_name.clone().unwrap_or_default()),
            last_name: Some(form.last_name.clone().unwrap_or_default()),
            email: Some(form.email.clone().unwrap_or_default()),
        };

        diesel::update(users.filter(user_id.eq(parsed_user_id)))
            .set(&updated_user)
            .execute(&mut conn)?;

        users
            .order(id.desc())
            .limit(1)
            .load::<models::User>(&mut conn)
    })
    .await
    .map_err(|_| UserError::UpdatingUser)?;

    match user_result {
        Ok(users_list) => Ok(HttpResponse::Ok().json(models::GenericResponse {
            status: "OK".to_string(),
            message: "User updated successfully".to_string(),
            data: Some(users_list),
        })),
        Err(diesel_error) => Err(UserError::from(UserError::DieselError(diesel_error))),
    }
}

// Delete user
pub async fn delete_user(
    pool: web::Data<DbPool>,
    path: web::Path<(String,)>,
) -> impl actix_web::Responder {
    let user_result = web::block(move || {
        let parsed_user_id = Uuid::parse_str(&path.into_inner().0).expect("Error parsing user_id");

        let mut conn = get_conn_from_db(pool);

        use crate::schema::users::dsl::*;
        
        diesel::delete(users.filter(user_id.eq(parsed_user_id))).execute(&mut conn)?;

        users
            .order(id.desc())
            .limit(1)
            .load::<models::User>(&mut conn)
    })
    .await
    .map_err(|_| UserError::DeletingUser)?;

    match user_result {
        Ok(users_list) => Ok(HttpResponse::Ok().json(models::GenericResponse {
            status: "OK".to_string(),
            message: "User Deleted successfully".to_string(),
            data: Some(users_list),
        })),
        Err(diesel_error) => Err(UserError::from(UserError::DieselError(diesel_error))),
    }
}
```
## Testing 

To test our application we can open any API platform such as postman or Insomnia.
Or you can use curl to test the API. (For non gui gang :D)

## Testing 
```bash
curl -X GET http://127.0.0.1:8080/
```
Respone:
```json
{
    "status": "OK",
    "message": "Working",
    "data": null
}
```
#### Create a user 
```bash
curl -X POST -H "Content-Type: application/json" -d '{"first_name":"John","last_name":"Doe","email":"johnDoe123@gmail.com"}' http://127.0.0.1:8080/add

curl -X POST -H "Content-Type: application/json" -d '{"first_name":"Jane","last_name":"Doe","email":"janeDoe@gmail.com"}' http://127.0.0.1:8080/add
```

Response: 
```json
{
    "status": "OK",
    "message": "User added successfully",
    "data": [
        {
            "id": 1,
            "user_id": "eb8f562b-637c-4653-9a0c-87d1079635a3",
            "first_name": "John",
            "last_name": "Doe",
            "email": "johnDoe123@gmail.com",
            "created_at": "2023-04-18T11:56:35.751709"
        }
    ]
}{
    "status": "OK",
    "message": "User added successfully",
    "data": [
        {
            "id": 2,
            "user_id": "af2b92fa-1aed-4ae6-9203-935b5e82d2b7",
            "first_name": "Jane",
            "last_name": "Doe",
            "email": "janeDoe@gmail.com",
            "created_at": "2023-04-18T11:56:35.765434"
        }
    ]
}
```

#### Get All users
```bash
curl -X GET http://127.0.0.1:8080/get
```
Response:

```json
{
    "status": "OK",
    "message": "Users Fetched successfully",
    "data": [
        {
            "id": 1,
            "user_id": "eb8f562b-637c-4653-9a0c-87d1079635a3",
            "first_name": "John",
            "last_name": "Doe",
            "email": "johnDoe123@gmail.com",
            "created_at": "2023-04-18T11:56:35.751709"
        },
        {
            "id": 2,
            "user_id": "af2b92fa-1aed-4ae6-9203-935b5e82d2b7",
            "first_name": "Jane",
            "last_name": "Doe",
            "email": "janeDoe@gmail.com",
            "created_at": "2023-04-18T11:56:35.765434"
        }
    ]
}
```

#### Get user by id
```bash
curl -X GET http://127.0.0.1:8080/get/af2b92fa-1aed-4ae6-9203-935b5e82d2b7
```
Respone:
```json
{
    "status": "OK",
    "message": "User Fetched successfully",
    "data": [
        {
            "id": 2,
            "user_id": "af2b92fa-1aed-4ae6-9203-935b5e82d2b7",
            "first_name": "Jane",
            "last_name": "Doe",
            "email": "janeDoe@gmail.com",
            "created_at": "2023-04-18T11:56:35.765434"
        }
    ]
}
```
#### Update user
```bash
curl -X POST -H "Content-Type: application/json" -d '{"first_name": "Charlie", "last_name":"Doe","email":"charlieDoe@gmail.com"}' http://127.0.0.1:8080/update/eb8f562b-637c-4653-9a0c-87d1079635a3
```
Respone: 
```json
{
    "status": "OK",
    "message": "User updated successfully",
    "data": [
        {
            "id": 2,
            "user_id": "af2b92fa-1aed-4ae6-9203-935b5e82d2b7",
            "first_name": "Charlie",
            "last_name": "Doe",
            "email": "charlieDoe@gmail.com",
            "created_at": "2023-04-18T11:56:35.765434"
        }
    ]
}
```

#### Delete user

```bash
curl -X GET http://127.0.0.1:8080/delete/{id}
```
Respone: 
```json
{
    "status": "OK",
    "message": "User Deleted successfully",
    "data": [
        {
            "id": 2,
            "user_id": "af2b92fa-1aed-4ae6-9203-935b5e82d2b7",
            "first_name": "Charlie",
            "last_name": "Doe",
            "email": "charlieDoe@gmail.com",
            "created_at": "2023-04-18T11:56:35.765434"
        }
    ]
}
```
Although this is a very basic implementaion, it demonstrates how to use Actix-web and Diesel to build a RESTful API. This could be used as a starting point for a more complex application.
