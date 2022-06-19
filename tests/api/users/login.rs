use crate::helpers::{spawn_app, User, MAX_PASSWORD_LENGTH, MIN_PASSWORD_LENGTH};
use fake::faker::internet::en::Password;
use fake::{Fake, Faker};
use reqwest::StatusCode;
use serde_json::Value;

#[tokio::test]
async fn successful_login() {
    let app = spawn_app().await;

    let user = app.create_new_user().await;
    let response = app.post_login(&user).await;
    assert_eq!(response.status(), StatusCode::OK);

    let json: Value = response.json().await.unwrap();
    assert_eq!(json["token"].as_str().unwrap().len(), 40);
}

#[tokio::test]
async fn different_tokens_on_login_with_same_user() {
    let app = spawn_app().await;

    let user = app.create_new_user().await;
    let first_token = app.login(&user).await;
    let second_token = app.login(&user).await;
    assert_ne!(first_token, second_token);
}

#[tokio::test]
async fn bad_request_on_invalid_credentials() {
    let app = spawn_app().await;

    let mut user = app.create_new_user().await;

    user.password = Password(MIN_PASSWORD_LENGTH..(MAX_PASSWORD_LENGTH + 1)).fake();
    let response = app.post_login(&user).await;
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    user.password = Password(0..MIN_PASSWORD_LENGTH).fake();
    let response = app.post_login(&user).await;
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let user: User = Faker.fake();
    let response = app.post_login(&user).await;
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}
