use actix_web::{HttpResponse, Responder, web};
use sqlx::{PgPool, query, types::chrono::Utc};
use tracing::{error, instrument};
use uuid::Uuid;

use crate::{NewSubscriber, SubscriberEmail, SubscriberName};

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

#[instrument(
    name = "Adding a new subscriber",
    skip(pool, form),
    fields(subscriber_email = %form.email, subscriber_name = %form.name)
)]
pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> impl Responder {
    let Ok(new_subscriber) = parse_subcriber(form.0) else {
        return HttpResponse::BadRequest();
    };

    match insert_subscriber(&pool, new_subscriber).await {
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::InternalServerError(),
    }
}

fn parse_subcriber(form: FormData) -> Result<NewSubscriber, String> {
    let name = SubscriberName::parse(form.name)?;
    let email = SubscriberEmail::parse(form.email)?;

    Ok(NewSubscriber { email, name })
}
#[instrument(
    name = "Saving new subscriber details in Postgres",
    skip(pool, new_subscriber)
)]
pub async fn insert_subscriber(
    pool: &PgPool,
    new_subscriber: NewSubscriber,
) -> Result<(), sqlx::Error> {
    query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        new_subscriber.email.as_ref(),
        new_subscriber.name.as_ref(),
        Utc::now()
    )
    .execute(pool)
    .await
    .map_err(|e| {
        error!("Failed to execute query: {:?}", e);
        e
    })?;

    Ok(())
}
