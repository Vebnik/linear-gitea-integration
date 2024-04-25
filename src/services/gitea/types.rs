use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Author {
    pub email: String,
    pub name: String,
    pub username: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Commit {
    pub added: Option<String>,
    pub author: Author,
    pub committer: Author,
    pub id: String,
    pub message: String,
    pub modified: Option<String>,
    pub removed: Option<String>,
    pub timestamp: String,
    pub url: String,
    pub verification: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Repository {
    // pub allow_merge_commits: bool,
    // pub allow_rebase: bool,
    // pub allow_rebase_explicit: bool,
    // pub allow_squash_merge: bool,
    // pub archived: bool,
    pub avatar_url: String,
    pub clone_url: String,
    pub created_at: String,
    pub default_branch: String,
    pub default_merge_style: String,
    pub description: String,
    // pub empty: bool,
    // pub fork: bool,
    pub forks_count: i32,
    pub full_name: String,
    // pub has_issues: bool,
    // pub has_projects: bool,
    // pub has_pull_requests: bool,
    // pub has_wiki: bool,
    pub html_url: String,
    pub id: i32,
    // pub ignore_whitespace_conflicts: bool,
    // pub internal: bool,
    // pub mirror: bool,
    pub mirror_interval: String,
    pub mirror_updated: String,
    pub name: String,
    pub open_issues_count: i32,
    pub open_pr_counter: i32,
    pub original_url: String,
    pub owner: User,
    // pub private: bool,
    pub release_counter: i32,
    pub repo_transfer: Option<String>,
    pub size: i32,
    pub ssh_url: String,
    pub stars_count: i32,
    // pub template: bool,
    pub updated_at: String,
    pub watchers_count: i32,
    pub website: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub active: bool,
    pub avatar_url: String,
    pub created: String,
    pub description: String,
    pub email: String,
    pub followers_count: i32,
    pub following_count: i32,
    pub full_name: String,
    pub id: i32,
    pub is_admin: bool,
    pub language: String,
    pub last_login: String,
    pub location: String,
    pub login: String,
    pub prohibit_login: bool,
    pub restricted: bool,
    pub starred_repos_count: i32,
    pub username: String,
    pub visibility: String,
    pub website: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Branch {
    pub label: String,
    pub sha: String,
    pub repo_id: i32,
    pub repo: Repository,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PullRequest {
    pub id: i32,
    pub url: String,
    pub number: i32,
    pub user: User,
    pub title: String,
    pub body: String,
    pub labels: Vec<String>,
    pub milestone: Option<String>,
    pub assignee: Option<User>,
    pub assignees: Option<Vec<User>>,
    pub state: String,
    pub is_locked: bool,
    pub comments: i32,
    pub html_url: String,
    pub diff_url: String,
    pub patch_url: String,
    pub mergeable: bool,
    pub merged: bool,
    pub merged_at: Option<String>,
    pub merge_commit_sha: Option<String>,
    pub merged_by: Option<String>,
    pub base: Branch,
    pub head: Branch,
    pub merge_base: String,
    pub due_date: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub closed_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommonEvent {
    pub after: String,
    pub before: String,
    pub commits: Vec<Commit>,
    pub compare_url: String,
    pub head_commit: Commit,
    pub pusher: User,
    pub repository: Repository,
    pub sender: User,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PullRequestEvent {
    pub action: String,
    pub number: i32,
    pub pull_request: PullRequest,
    pub repository: Repository,
    pub sender: User,
    pub review: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct BranchData {
    pub team_key: String,
    pub number: i32,
    pub title: String,
}

#[derive(Debug, Clone)]
pub struct Client {
    pub endpoint: String,
    pub key: String,
}
