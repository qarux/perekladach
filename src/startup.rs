use crate::middlewares;
use crate::routes::logout::logout;
use crate::routes::{
    all_projects, chapters, login::login, new_chapter, new_project, new_user, project,
    source_image, translated_image,
};
use actix_web::dev::Server;
use actix_web::web::service;
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
                            .route("", web::get().to(all_projects))
                            .route("", web::post().to(new_project))
                            .service(project)
                            .service(
                                web::scope("/{project_slug}/chapters")
                                    .route("", web::get().to(chapters))
                                    .route("", web::post().to(new_chapter)),
                            ),
                    )
                    .service(
                        web::scope("/chapters")
                            .route("/{project_slug}", web::get().to(chapters))
                            .route("", web::post().to(new_chapter)),
                    )
                    .service(
                        web::scope("/pages")
                            // TODO
                            .route("/{uuid}/source", web::get().to(source_image))
                            .route("/{uuid}/translated", web::get().to(translated_image)),
                    ),
            )
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
