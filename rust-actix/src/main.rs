use actix_web::{middleware, web, App, HttpServer};
use dotenv::dotenv;
use log::error;
use service::{Envs, TracksService};
use sqlx::MySqlPool;
use std::env;

mod error;
mod handlers;
mod service;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv().ok();
    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let db_pool: MySqlPool = match MySqlPool::new(&database_url).await {
        Ok(val) => Some(val),
        Err(err) => {
            error!("{}", err);
            None
        }
    }
    .unwrap();

    let envs = Envs {
        service_name: env::var("TRACKS_SERVICE_NAME")
            .expect("TRACKS_SERVICE_NAME is not set in the .env file"),
        node_id: uuid::Uuid::new_v4().to_string(),
    };

    let _ = TracksService::register(&db_pool, envs.clone());

    HttpServer::new(move || {
        App::new()
            .data(db_pool.clone())
            .data(envs.clone())
            .wrap(middleware::Logger::default())
            .default_service(web::route().to(handlers::not_found))
            .configure(handlers::init)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::dev::Service;
    use actix_web::{http, test, App, Error};

    #[actix_rt::test]
    async fn test_health_check() -> Result<(), Error> {
        let mut app = test::init_service(App::new().configure(handlers::init)).await;

        let req = test::TestRequest::get().uri("/health").to_request();

        let resp = app.call(req).await.unwrap();

        assert_eq!(resp.status(), http::StatusCode::OK);

        Ok(())
    }
}
