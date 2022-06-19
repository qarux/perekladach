use crate::helpers::{spawn_app, User, MIN_PASSWORD_LENGTH};
use fake::faker::internet::en::Password;
use fake::{Fake, Faker};
use reqwest::StatusCode;

#[tokio::test]
async fn successful_user_creation() {
    let app = spawn_app().await;

    for _ in 0..5 {
        app.create_new_user().await;
    }
}

#[tokio::test]
async fn bad_request_on_invalid_or_missing_parameters() {
    let app = spawn_app().await;

    let mut user: User = Faker.fake();
    user.password = Password(1..MIN_PASSWORD_LENGTH).fake();
    let response = app.post_user(&user).await;
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let user = User {
        username: "".to_string(),
        password: "".to_string(),
    };
    let response = app.post_user(&user).await;
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn bad_request_on_existing_user() {
    let app = spawn_app().await;

    let user = app.create_new_user().await;
    let response = app.post_user(&user).await;
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}
