use crate::helpers::{AuthenticatedApp};
use reqwest::StatusCode;

#[tokio::test]
async fn successful_logout() {
    let app = AuthenticatedApp::new().await;
    let response = app.post_logout().await;
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn bad_request_on_wrong_token() {
    let app = AuthenticatedApp::new().await;
    let response = app.post_logout().await;
    assert_eq!(response.status(), StatusCode::OK);

    let response = app.post_logout().await;
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}