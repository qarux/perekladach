use crate::middlewares;
use crate::routes::logout::logout;
use crate::routes::{login::login, new_project, new_user, project};
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use sqlx::PgPool;
use std::net::TcpListener;

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, anyhow::Error> {
    let db_pool = web::Data::new(db_pool);

    let server = HttpServer::new(move || {
        App::new()
            .service(
                web::scope("/users")
                    .route("", web::post().to(new_user))
                    .route("/login", web::post().to(login))
                    .route(
                        "/logout",
                        web::post()
                            .to(logout)
                            .wrap(HttpAuthentication::bearer(middlewares::auth::validator)),
                    ),
            )
            .service(
                web::scope("")
                    .wrap(HttpAuthentication::bearer(middlewares::auth::validator))
                    .service(
                        web::scope("/projects")
                            .route("", web::post().to(new_project))
                            .service(project)
                    )
            )
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}