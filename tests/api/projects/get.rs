use crate::helpers::{AuthenticatedApp, Project};
use reqwest::StatusCode;

#[tokio::test]
async fn get_all_projects() {
    let app = AuthenticatedApp::new().await;
    let mut requests_data = vec![];
    for _ in 0..10 {
        requests_data.push(app.create_project().await);
    }

    let response = app.get_projects().await;
    assert_eq!(response.status(), StatusCode::OK);
    let projects: Vec<Project> = response.json().await.unwrap();
    assert_eq!(projects.len(), requests_data.len());
    assert!(projects
        .iter()
        .all(|project| requests_data.iter().any(|data| data.slug == project.slug)));
}

#[tokio::test]
async fn get_project() {
    let app = AuthenticatedApp::new().await;

    let request_data = app.create_project().await;
    let response = app.get_project(&request_data.slug).await;
    assert_eq!(response.status(), StatusCode::OK);
    let project: Project = response.json().await.unwrap();
    assert_eq!(project.slug, request_data.slug);
}

#[tokio::test]
async fn not_found_on_not_existing_project() {
    let app = AuthenticatedApp::new().await;

    let request_data = app.create_project().await;
    let response = app.get_project(&format!("{}{}", request_data.slug, 1)).await;
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}