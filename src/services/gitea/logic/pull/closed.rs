use crate::error::{CustomError, Result};
use crate::services::gitea::types::PullRequestEvent;
use crate::services::gitea::utils::extract_branch_data;
use crate::services::linear::types::{Issue, State};

pub async fn close_pr_flow(event: PullRequestEvent) -> Result<()> {
    log::info!("PR closed action");

    let data = extract_branch_data(event.pull_request.head.label).await?;
    let try_issue = Issue::get_by_branch(data.clone()).await?;

    match try_issue {
        Some(issue) => {
            log::info!("{:#?}", &issue);

            let state = State::get_all_by_team(issue.team.clone())
                .await?
                .iter()
                .find(|el| el.name.eq("Done"))
                .ok_or(|| CustomError::UbExtendError("Not found state".to_string()))
                .map_err(|_| CustomError::UbExtendError("Not found state".to_string()))?
                .to_owned();

            issue.update_state(state).await?;
        }
        None => log::error!("Not found linear task for: {:?}", data),
    };

    log::info!("{:#?}", data);

    Ok(())
}
