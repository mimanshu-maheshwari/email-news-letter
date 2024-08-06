
use chrono::Utc;
use uuid::Uuid;

use actix_web::{web, HttpResponse, Responder};
use sqlx::PgConnection;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String, 
    name: String
}

pub async fn subscriptions(form: web::Form<FormData>, connection: web::Data<PgConnection>) -> impl Responder {
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#, 
        Uuid::new_v4(), 
        form.email, 
        form.name, 
        Utc::now(),
        )
        .execute(connection.get_ref())
        .await;
    HttpResponse::Ok().finish()
}


