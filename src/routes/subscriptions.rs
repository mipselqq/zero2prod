use actix_web::{HttpResponse, Responder, web};
use sqlx::{PgPool, types::chrono::Utc};
use tracing::info;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(form: web::Form<FormData>, connection: web::Data<PgPool>) -> impl Responder {
    let name = &form.name;
    let email = &form.email;

    tracing::info!("Adding '{email}' '{name}' as new subsciber");
    if email.is_empty() || name.is_empty() {
        return HttpResponse::BadRequest();
    }

    tracing::info!("Saving new subscriber in Postgres");
    let query_result = sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        email,
        name,
        Utc::now()
    )
    .execute(connection.get_ref())
    .await;

    match query_result {
        Ok(_) => {
            tracing::info!("Saved new subscriber in Postgres");
            HttpResponse::Ok()
        }
        Err(e) => {
            tracing::error!("Failed to execute query: {e:?}",);
            HttpResponse::InternalServerError()
        }
    }
}
