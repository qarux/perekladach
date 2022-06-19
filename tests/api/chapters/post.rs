use crate::helpers::AuthenticatedApp;

#[tokio::test]
async fn successful_chapter_creation() {
    let app = AuthenticatedApp::new().await;
    let project_slug = app.create_project().await.slug;

    for _ in 0..5 {
        app.create_chapter(&project_slug).await;
    }
}
