use fake::faker::address::en::CountryCode;
use fake::faker::internet::en::{Password, Username};
use fake::faker::name::en::Title;
use fake::Fake;
use fake::{Dummy, Faker, StringFaker};
use perekladach::startup;
use reqwest::{header, Client, Response, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions, PgSslMode};
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::TcpListener;
use url::Url;
use uuid::Uuid;

pub const MIN_PASSWORD_LENGTH: usize = 8;
pub const MAX_PASSWORD_LENGTH: usize = 64;

pub struct TestApp {
    pub api_client: Client,
    pub address: String,
}

pub struct AuthenticatedApp {
    app: TestApp,
    token: Token,
}

#[derive(Debug, Serialize, Dummy)]
pub struct User {
    #[dummy(faker = "Username()")]
    pub username: String,
    #[dummy(faker = "Password(MIN_PASSWORD_LENGTH..(MAX_PASSWORD_LENGTH + 1))")]
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct Project {
    pub slug: String,
    pub name: String,
    pub source_language: String,
    pub target_language: String,
    pub chapters_count: u32,
    pub translation_progress: f32,
}

#[derive(Debug, Serialize)]
pub struct CreateProjectRequestData {
    pub slug: String,
    pub name: String,
    pub source_language: String,
    pub target_language: String,
}

#[derive(Debug, Deserialize)]
pub struct Chapter {
    pub index: f32,
    pub title: String,
    pub page_count: u32,
    translation_progress: f32,
    project_slug: String,
}

#[derive(Debug, Serialize)]
pub struct CreateChapterRequestData {
    pub index: Option<f32>,
    pub title: String,
    pub project_slug: String,
}

#[derive(Debug, Deserialize)]
pub struct Page {
    index: u32,
    thumbnail_url: Url,
    source_image_url: Url,
    draft_editor_data_url: Option<Url>,
    translated_image_url: Option<Url>,
}

pub type Token = String;

impl TestApp {
    pub async fn post_user(&self, user: &User) -> Response {
        self.api_client
            .post(&format!("{}{}", self.address, "/users"))
            .json(user)
            .send()
            .await
            .unwrap()
    }

    pub async fn post_login(&self, user: &User) -> Response {
        self.api_client
            .post(&format!("{}{}", self.address, "/users/login"))
            .json(user)
            .send()
            .await
            .unwrap()
    }

    pub async fn create_new_user(&self) -> User {
        let user: User = Faker.fake();
        let response = self.post_user(&user).await;
        assert_eq!(response.status(), StatusCode::CREATED);

        user
    }

    pub async fn login(&self, user: &User) -> Token {
        let response = self.post_login(&user).await;
        assert_eq!(response.status(), StatusCode::OK);

        let json: Value = response.json().await.unwrap();
        json["token"].as_str().unwrap().to_string()
    }
}

impl AuthenticatedApp {
    pub async fn new() -> Self {
        let mut app = spawn_app().await;
        let user = app.create_new_user().await;
        let token = app.login(&user).await;

        let mut headers = header::HeaderMap::new();
        let mut auth_value = header::HeaderValue::from_str(&format!("Bearer {}", token)).unwrap();
        auth_value.set_sensitive(true);
        headers.insert(header::AUTHORIZATION, auth_value);
        app.api_client = Client::builder().default_headers(headers).build().unwrap();

        AuthenticatedApp { app, token }
    }

    pub async fn post_logout(&self) -> Response {
        self.app
            .api_client
            .post(&format!("{}{}", self.app.address, "/users/logout"))
            .bearer_auth(&self.token)
            .send()
            .await
            .unwrap()
    }

    pub async fn post_projects(&self, data: &CreateProjectRequestData) -> Response {
        self.app
            .api_client
            .post(&format!("{}{}", self.app.address, "/projects"))
            .json(data)
            .send()
            .await
            .unwrap()
    }

    pub async fn get_projects(&self) -> Response {
        self.app
            .api_client
            .get(&format!("{}{}", self.app.address, "/projects"))
            .send()
            .await
            .unwrap()
    }

    pub async fn get_project(&self, project_name: &str) -> Response {
        self.app
            .api_client
            .get(&format!(
                "{}{}{}",
                self.app.address, "/projects/", project_name
            ))
            .send()
            .await
            .unwrap()
    }

    pub async fn post_chapter(
        &self,
        data: &CreateChapterRequestData,
        project_slug: &str,
    ) -> Response {
        self.app
            .api_client
            .post(&format!(
                "{}/projects/{}/chapters",
                self.app.address, project_slug
            ))
            .json(data)
            .send()
            .await
            .unwrap()
    }

    pub async fn get_chapters(&self, project_slug: &str) -> Response {
        self.app
            .api_client
            .get(&format!(
                "{}/projects/{}/chapters",
                self.app.address, project_slug
            ))
            .send()
            .await
            .unwrap()
    }

    pub async fn post_pages(&self, project_name: &str, chapter_index: f32) -> Response {
        self.app
            .api_client
            .post(&format!(
                "{}/pages/{}/{}",
                self.app.address, project_name, chapter_index
            ))
            .send()
            .await
            .unwrap()
    }

    pub async fn get_pages(&self, project_name: &str, chapter_index: f32) -> Response {
        self.app
            .api_client
            .get(&format!(
                "{}/pages/{}/{}",
                self.app.address, project_name, chapter_index
            ))
            .send()
            .await
            .unwrap()
    }

    pub async fn create_project(&self) -> CreateProjectRequestData {
        let data = CreateProjectRequestData {
            slug: StringFaker::with(Vec::from("0123456789abcdefghijklmnopqrstuvwxyz"), 8..12)
                .fake(),
            name: Title().fake(),
            // TODO Fake language codes
            source_language: "jp".to_string(),
            target_language: "uk".to_string(),
        };
        let response = self.post_projects(&data).await;
        assert_eq!(response.status(), StatusCode::CREATED);

        data
    }

    pub async fn create_chapter(&self, project_slug: &str) -> CreateChapterRequestData {
        let data = CreateChapterRequestData {
            index: None,
            title: Title().fake(),
            project_slug: project_slug.to_owned(),
        };
        let response = self.post_chapter(&data, project_slug).await;
        assert_eq!(response.status(), StatusCode::CREATED);

        data
    }

    pub async fn create_page(&self, _project_name: &str, _chapter_index: f32) {
        todo!()
    }
}

pub async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind address");
    let addr = listener.local_addr().unwrap();
    let db_pool = configure_database().await;
    tokio::spawn(async { startup::run(listener, db_pool).unwrap().await });

    let api_client = Client::new();
    let address = format!("http://localhost:{}", addr.port());
    TestApp {
        api_client,
        address,
    }
}

async fn configure_database() -> PgPool {
    // TODO Read config file
    let database_name = Uuid::new_v4().to_string();
    let db_config = PgConnectOptions::new()
        .username("admin")
        .password("password")
        .host("127.0.0.1")
        .port(5432)
        .database("perekladach")
        .ssl_mode(PgSslMode::Prefer);
    let mut connection = PgConnection::connect_with(&db_config)
        .await
        .expect("Failed to connect to Postgres");
    connection
        .execute(&*format!(r#"CREATE DATABASE "{}";"#, database_name))
        .await
        .expect("Failed to create database");

    let db_config = db_config.database(&database_name);
    let pool = PgPoolOptions::new()
        .connect_with(db_config)
        .await
        .expect("Failed to connect to Postgres");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to migrate database");

    pool
}
