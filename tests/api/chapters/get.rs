use crate::helpers::{AuthenticatedApp, Chapter};
use reqwest::StatusCode;

#[tokio::test]
async fn get_all_chapters_in_project() {
    let app = AuthenticatedApp::new().await;
    let project_slug = app.create_project().await.slug;
    let mut requests_data = vec![];
    for _ in 0..10 {
        requests_data.push(app.create_chapter(&project_slug).await);
    }

    let response = app.get_chapters(&project_slug).await;
    assert_eq!(response.status(), StatusCode::OK);
    let mut chapters: Vec<Chapter> = response.json().await.unwrap();
    assert_eq!(chapters.len(), requests_data.len());

    requests_data.sort_by(|a, b| a.title.cmp(&b.title));
    chapters.sort_by(|a, b| a.title.cmp(&b.title));
    assert!(requests_data
        .into_iter()
        .zip(chapters.into_iter())
        .all(|(data, chapter)| data.title == chapter.title));
}
