use chrono::Utc;
use uuid::Uuid;

use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;

use crate::domain::{NewSubscriber, SubscriberName};


/// This Struct contains email id and name of the subscriber.
#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

#[tracing::instrument(
    name="Adding a new subscriber", 
    skip(form, pool), 
    fields(
        subscriber_email = %form.email, 
        subscriber_name = %form.name,
    )
)]
/// call method to insert subscriber detail in db and also initiate tracing.
pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> impl Responder {
    let name = match SubscriberName::parse(form.0.name) {
        Ok(name) => name,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };
    let new_subscriber = NewSubscriber {
        email:form.0.email, 
        name,
    };

    match insert_subscriber(&pool, &new_subscriber).await 
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            tracing::error!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

/// Add subscriber details in db and handle the query as well.
#[tracing::instrument(
    name = "Saving new subscriber detials in the database",
    skip(new_subscriber, pool)
)]
pub async fn insert_subscriber(pool: &PgPool, new_subscriber: &NewSubscriber) -> Result<(), sqlx::Error> {
sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        new_subscriber.email,
        new_subscriber.name.as_ref(),
        Utc::now().naive_utc(),
    )
    .execute(pool)
    .await 
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(())
}
