use crate::error::{CustomError, Result};
use crate::services::gitea::types::PullRequestEvent;
use crate::services::gitea::utils::extract_branch_data;
use crate::services::linear::types::{Issue, State};

pub async fn open_pr_flow(event: PullRequestEvent) -> Result<()> {
    log::info!("PR opened flow");

    let data = extract_branch_data(event.pull_request.head.label.clone()).await?;
    let try_issue = Issue::get_by_branch(data.clone()).await?;

    match try_issue {
        Some(issue) => {
            log::debug!("{:#?}", &issue);

            let state = State::get_all_by_team(issue.team.clone())
                .await?
                .iter()
                .find(|el| el.name.eq("In Review"))
                .ok_or(|| CustomError::UbExtendError("Not found state".to_string()))
                .map_err(|_| CustomError::UbExtendError("Not found state".to_string()))?
                .to_owned();

            issue.update_state(state).await?;
            event.make_comment(&format!("[Linked to task]({})", issue.url)).await?;
        }
        None => log::error!("Not found linear task for: {:#?}", data),
    };

    Ok(())
}