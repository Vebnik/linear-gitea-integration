
#[tokio::test]
async fn test_query_filters() {
    use crate::services::linear::types::Client;

    dotenv::dotenv().expect("Error on read .env");
    env_logger::init();

    let endpoint = std::env::var("LINEAR_URL").unwrap();
    let key = std::env::var("LINEAR_KEY").unwrap();

    let client = Client::new(endpoint, key);

    let query = r#"
        query Team {
            team(id: "5557a5eb-b9cb-483d-89bd-8ff21073a6c6") {
            id
            name
        
            issues {
                nodes {
                id
                title
                assignee {
                    id
                    name
                }
                createdAt
                archivedAt
                }
            }
            }
        }
    "#;

    let result = client.execute(query.to_string())
        .await.unwrap();

    assert_eq!(&result["data"]["team"]["id"], "5557a5eb-b9cb-483d-89bd-8ff21073a6c6");
}

#[tokio::test]
async fn test_query_issue() {
    use crate::services::linear::types::Issue;

    dotenv::dotenv().expect("Error on read .env");
    env_logger::init();

    let team = "5557a5eb-b9cb-483d-89bd-8ff21073a6c6".to_string();

    let issues = Issue::get_all_by_team(team).await.unwrap();

    assert!(dbg!(issues.len()) > 1)
}

#[tokio::test]
async fn test_query_team() {
    use crate::services::linear::types::Team;

    dotenv::dotenv().expect("Error on read .env");
    env_logger::init();

    let teams = Team::get_all().await.unwrap();

    dbg!(&teams);

    assert!(dbg!(teams.len()) > 1)
}

#[tokio::test]
async fn test_query_issue_by_title() {
    use crate::services::linear::types::Issue;
    use crate::services::gitea::types::BranchData;

    dotenv::dotenv().expect("Error on read .env");
    env_logger::init();

    let brench_data = BranchData {
        title: "partial payments".to_string(),
        number: 60,
        team_key: "PAY".to_string()
    };

    let issues = Issue::get_by_branch(brench_data).await.unwrap();

    match issues {
        Some(issue) => {
            dbg!(issue);
            assert!(true);
        },
        None => assert!(false)
    }
}

#[tokio::test]
async fn test_query_state_by_team() {
    use crate::services::linear::types::{Team, State};

    dotenv::dotenv().expect("Error on read .env");
    env_logger::init();

    let team = Team::get_all()
        .await.unwrap().first().unwrap().to_owned();

    let state = State::get_all_by_team(team)
        .await.unwrap()
        .iter().find(|el| el.name.eq("In Review"))
        .unwrap().to_owned();

    dbg!(&state);

    assert_eq!(state.issue_type, "started")
}