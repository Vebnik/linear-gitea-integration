use actix_web::{web::Json, HttpRequest, HttpResponse};
use serde_json::Value;

use super::types::PullRequestEvent;
use crate::error::{CustomError, Result};
use crate::services::gitea::logic::pull::on_pull_request_event;

pub async fn test(data: Json<Value>, req: HttpRequest) -> Result<HttpResponse> {
    let raw_data: Value = data.clone();

    let event_type = req
        .headers()
        .get("X-Gitea-Event")
        .ok_or(|err| CustomError::UbExtendError(err))
        .map_err(|_| CustomError::UbError)?
        .to_str()
        .map_err(|err| CustomError::UbExtendError(err.to_string()))?;

    match event_type {
        "pull_request" => {
            let event: PullRequestEvent = serde_json::from_value(raw_data)
                .map_err(|err| CustomError::UbExtendError(err.to_string()))?;

            on_pull_request_event(event).await?;
        }
        _ => {
            log::error!("Not expected event: {:?}", event_type);
        }
    }

    Ok(HttpResponse::Ok().json(data))
}
