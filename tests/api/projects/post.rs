use reqwest::StatusCode;
use crate::helpers::{AuthenticatedApp};

#[tokio::test]
async fn successful_project_creation() {
    let app = AuthenticatedApp::new().await;

    app.create_project().await;
    app.create_project().await;
}

#[tokio::test]
async fn bad_request_on_existing_project() {
    let app = AuthenticatedApp::new().await;

    let request_data = app.create_project().await;
    let response = app.post_projects(&request_data).await;
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}