use crate::{models, user_error::UserError, DbPool};
use actix_web::{web, HttpResponse, Responder};
use chrono::prelude::*;
use diesel::prelude::*;
use uuid::Uuid;

pub async fn health_checker() -> impl Responder {

    let response = models::GenericResponse::<()> {
        status: "OK".to_string(),
        message: "Working".to_string(),
        data: None,
    };
    HttpResponse::Ok().json(response)
}

fn get_conn_from_db(
    pool: web::Data<diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<PgConnection>>>,
) -> diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<PgConnection>> {
    let conn = pool
        .get()
        .expect("Error getting a connection from the pool");
    conn
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
            message: "Users added successfully".to_string(),
            data: Some(users_list),
        })),
        Err(diesel_error) => Err(UserError::from(UserError::DieselError(diesel_error))),
    }
}

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
            message: "Users updated successfully".to_string(),
            data: Some(users_list),
        })),
        Err(diesel_error) => Err(UserError::from(UserError::DieselError(diesel_error))),
    }
}

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
            message: "Users Deleted successfully".to_string(),
            data: Some(users_list),
        })),
        Err(diesel_error) => Err(UserError::from(UserError::DieselError(diesel_error))),
    }
}