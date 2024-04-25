use crate::error::Result;
use crate::services::gitea::logic::pull::closed::close_pr_flow;
use crate::services::gitea::logic::pull::opened::open_pr_flow;
use crate::services::gitea::types::PullRequestEvent;

pub mod closed;
pub mod opened;

pub async fn on_pull_request_event(event: PullRequestEvent) -> Result<()> {
    log::info!("PR Event");

    match event.action.as_str() {
        "closed" => close_pr_flow(event).await?,
        "opened" => open_pr_flow(event).await?,
        _ => log::error!("UB pull request event action"),
    }

    Ok(())
}
