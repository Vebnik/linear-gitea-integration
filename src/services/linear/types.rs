use serde::{Deserialize, Serialize};


#[derive(Debug)]
pub struct Client {
    pub endpoint: String,
    pub key: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Assignee {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct State {
    #[serde(alias = "type")]
    pub issue_type: String,
    pub id: String,
    pub name: String
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub struct Issue {
    pub id: String,
    pub title: String,
    pub created_at: String,
    pub branch_name: String,
    pub url: String,
    pub assignee: Option<Assignee>,
    pub archived_at: Option<String>,
    pub state: State,
    pub team: Team,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Team {
    pub id: String,
    pub name: String,
    pub key: String,
}