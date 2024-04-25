use reqwest::Method;
use serde_json::{json, Value};

use super::types::{Client, PullRequestEvent};
use crate::error::{CustomError, Result};

impl Client {
    pub async fn default() -> Result<Self> {
        let endpoint = std::env::var("GITEA_URL")
            .map_err(|_| CustomError::UbExtendError("Not found GITEA_URL".to_string()))?;

        let key = std::env::var("GITEA_KEY")
            .map_err(|_| CustomError::UbExtendError("Not found GITEA_KEY".to_string()))?;

        Ok(Self { endpoint, key })
    }

    pub async fn execute(
        &self,
        endpoint: &str,
        method: Method,
        body: Option<Value>,
    ) -> Result<Value> {
        let url = format!("{}{}?token={}", &self.endpoint, endpoint, &self.key);
        let req_client = reqwest::Client::new();

        let result = match method {
            Method::POST => req_client
                .post(url)
                .header("Content-Type", "application/json")
                .body(body.unwrap().to_string())
                .send()
                .await
                .map_err(|_| CustomError::UbExtendError("Error in gitea client".to_string()))?
                .json()
                .await
                .map_err(|_| CustomError::UbExtendError("Error in gitea client".to_string()))?,
            Method::GET => req_client
                .get(url)
                .send()
                .await
                .map_err(|_| CustomError::UbExtendError("Error in gitea client".to_string()))?
                .json()
                .await
                .map_err(|_| CustomError::UbExtendError("Error in gitea client".to_string()))?,
            _ => {
                log::error!("Unexpected gitea client method");
                Err(CustomError::UbExtendError(
                    "Unexpected gitea client method".to_string(),
                ))?
            }
        };

        log::debug!("{:#?}", &result);

        Ok(result)
    }
}

impl PullRequestEvent {
    pub async fn make_comment(&self, text: &str) -> Result<Value> {
        let client = Client::default().await?;

        let endpoint = format!(
            "repos/{}/{}/pulls/{}/reviews",
            &self.repository.owner.login, &self.repository.name, &self.pull_request.number
        );

        let body = json!({"body": text.to_string()});

        Ok(client.execute(&endpoint, Method::POST, Some(body)).await?)
    }
}
