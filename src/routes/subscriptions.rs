use actix_web::{HttpResponse, Responder, web};
use sqlx::{PgPool, query, types::chrono::Utc};
use tracing::{debug, error, info};
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(connection, form),
    fields(subscriber_email = %form.email, subscriber_name = %form.name)
)]
pub async fn subscribe(form: web::Form<FormData>, connection: web::Data<PgPool>) -> impl Responder {
    let subscriber_name = &form.name;
    let subscriber_email = &form.email;

    if subscriber_email.is_empty() || subscriber_name.is_empty() {
        return HttpResponse::BadRequest();
    }

    debug!("Saving new subscriber in Postgres");
    let query_result = query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        subscriber_email,
        subscriber_name,
        Utc::now()
    )
    .execute(connection.get_ref())
    .await;

    match query_result {
        Ok(_) => {
            debug!("Saved new subscriber in Postgres");
            HttpResponse::Ok()
        }
        Err(e) => {
            error!("Failed to execute query: {e:?}",);
            HttpResponse::InternalServerError()
        }
    }
}
