use actix_web::{web::Json, HttpRequest, HttpResponse};
use serde_json::Value;

use crate::error::{Result, CustomError};
use crate::services::linear::types::{Issue, State};
use super::types::PullRequestEvent;
use super::utils::extract_branch_data;

async fn on_pr_event(event: PullRequestEvent) -> Result<()> {
    match event.action.as_str() {
        "closed" => {
            log::info!("PR closed action");

            let data = extract_branch_data(event.pull_request.head.label).await?;

            log::info!("{:?}", data);
        },
        "opened" => {
            log::info!("PR opened action");

            let data = extract_branch_data(event.pull_request.head.label).await?;
            let try_issue = Issue::get_by_branch(data.clone()).await?;

            match try_issue {
                Some(issue) => {
                    log::info!("{:?}", &issue);

                    let state = State::get_all_by_team(issue.team.clone())
                        .await?
                        .iter().find(|el| el.name.eq("In Review"))
                        .ok_or(|| CustomError::UbExtendError("Not found state".to_string()))
                        .map_err(|_| CustomError::UbExtendError("Not found state".to_string()))?
                        .to_owned();

                    issue.update_state(state).await?;
                },
                None => log::error!("Not found linear task for: {:?}", data)
                
            }
        },
        _ => log::info!("UB PullRequestEvent action"),
    }

    Ok(())
}

pub async fn test(data: Json<Value>, req: HttpRequest) -> Result<HttpResponse> {
    let raw_data: Value = data.clone();

    let event_type = req.headers().get("X-Gitea-Event")
        .ok_or(|| CustomError::UbError)
        .map_err(|_| CustomError::UbError)?
        .to_str()
        .map_err(|_| CustomError::UbError)?;

    match event_type {
        "pull_request" => {
            log::info!("PR Event");

            let event: PullRequestEvent = serde_json::from_value(raw_data)
                .map_err(|err| CustomError::UbExtendError(err.to_string()))?;

            on_pr_event(event).await?;
        },
        _ => {
            log::info!("UB Event");
        },
    }

    Ok(HttpResponse::Ok().json(data))
}