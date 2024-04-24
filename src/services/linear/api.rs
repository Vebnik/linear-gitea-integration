use serde_json::{json, Value};

use super::types::{Client, Issue, State, Team};
use crate::{error::{CustomError, Result}, services::gitea::types::BranchData};

impl Client {
    pub fn new(endpoint: String, key: String) -> Self {
        Self { endpoint, key }
    }

    pub async fn execute(&self, query: String) -> Result<Value> {
        let payload = json!({"query": query});

        let response = reqwest::Client::new()
            .post(&self.endpoint)
            .header("Authorization", &self.key)
            .header("Content-Type", "application/json")
            .body(payload.to_string())
            .send()
            .await.map_err(|_| CustomError::UbError)?
            .json::<serde_json::Value>()
            .await.map_err(|_| CustomError::UbError)?;

        Ok(response)
    }
}

impl Issue {
    pub async fn get_all_by_team(team: String) -> Result<Vec<Self>> {
        let endpoint = std::env::var("LINEAR_URL").unwrap();
        let key = std::env::var("LINEAR_KEY").unwrap();

        let client = Client::new(endpoint, key);

        let query = format!(r#"
            query Team {{
                team(id: "{}") {{
                id
                name
            
                issues {{
                    nodes {{
                        id
                        title
                        createdAt
                        branchName
                        url
                        archivedAt
                        assignee {{
                            id
                            name
                        }}
                        state {{
                            type
                        }}
                    }}
                }}
                }}
            }}
        "#, team);

        let mut result = client.execute(query.to_string()).await?;
        let nodes_value = result["data"]["team"]["issues"]["nodes"].take();

        let data = serde_json::from_value::<Vec<Self>>(nodes_value)
            .map_err(|err| CustomError::UbExtendError(err.to_string()))?;

        Ok(data)
    }

    pub async fn get_by_branch(branch: BranchData) -> Result<Option<Self>> {
        let endpoint = std::env::var("LINEAR_URL").unwrap();
        let key = std::env::var("LINEAR_KEY").unwrap();
        let client = Client::new(endpoint, key);

        let query = format!(r#"
            query Issues {{
                issues(filter: {{
                    title: {{containsIgnoreCase: "{}" }},
                    number: {{eq: {} }},
                    team: {{key: {{eqIgnoreCase: "{}" }}}}
                }}) {{
                    nodes {{
                        id
                        title
                        createdAt
                        branchName
                        url
                        archivedAt
                        assignee {{
                            id
                            name
                        }}
                        state {{
                            type
                            id
                            name
                        }}
                        team {{
                            id
                            name
                            key
                        }}
                    }}
                }}
            }}
        "#, branch.title, branch.number, branch.team_key);

        let mut result = client.execute(query.to_string()).await?;
        let nodes_value = result["data"]["issues"]["nodes"].take();

        let data = serde_json::from_value::<Vec<Self>>(nodes_value)
            .map_err(|err| CustomError::UbExtendError(err.to_string()))?;

        if data.is_empty() {
            return Ok(None);
        } else {
            return Ok(Some(data[0].clone()));
        }
    }

    pub async fn update_state(&self, state: State) -> Result<Self> {
        let endpoint = std::env::var("LINEAR_URL").unwrap();
        let key = std::env::var("LINEAR_KEY").unwrap();
        let client = Client::new(endpoint, key);

        let query = format!(r#"
            mutation Mutation {{
                issueUpdate(
                    id: "{}",
                    input: {{ stateId: "{}"}}
                ) {{
                    issue {{ id }}
                }}
            }}
        "#, &self.id, state.id);

        let mut result = client.execute(query.to_string()).await?;
        let nodes_value = result["data"]["issue"].take();

        Ok(self.clone())
    }
}

impl Team {
    pub async fn get_all() -> Result<Vec<Self>> {
        let endpoint = std::env::var("LINEAR_URL").unwrap();
        let key = std::env::var("LINEAR_KEY").unwrap();

        let client = Client::new(endpoint, key);

        let query = r#"
            query Teams {
                teams {
                    nodes {
                        id
                        name
                        key
                    }
                }
            }
        "#;

        let mut result = client.execute(query.to_string()).await?;
        let nodes_value = result["data"]["teams"]["nodes"].take();

        let data = serde_json::from_value::<Vec<Self>>(nodes_value)
            .map_err(|_| CustomError::UbError)?;

        Ok(data)
    }
}

impl State {
    pub async fn get_all_by_team(team: Team) -> Result<Vec<Self>> {
        let endpoint = std::env::var("LINEAR_URL").unwrap();
        let key = std::env::var("LINEAR_KEY").unwrap();
        let client = Client::new(endpoint, key);

        let query = format!(r#"
            query Query {{
                workflowStates(filter: {{
                    team: {{id: {{eq: "{}" }}}},
                }}) {{
                    nodes {{
                        id,
                        name,
                        type,
                    }}
                }}
            }}
        "#, team.id);

        let mut result = client.execute(query.to_string()).await?;
        let nodes_value = result["data"]["workflowStates"]["nodes"].take();

        let data = serde_json::from_value::<Vec<Self>>(nodes_value)
            .map_err(|err| CustomError::UbExtendError(err.to_string()))?;

        Ok(data)
    }
}