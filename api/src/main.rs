pub mod routes;
use std::{sync::{Mutex, Arc}, thread, time};
use actix_web::{App, HttpServer, web};
use routes::health;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_readiness = web::Data::new(health::AppReadiness {
        is_ready: Mutex::new(false),
    });
    let readiness_arc = Arc::clone(&app_readiness);
    thread::spawn(move || {
        thread::sleep(time::Duration::from_millis(10000));
        let mut is_ready = readiness_arc.is_ready.lock().unwrap();
        *is_ready = true;
    });
    HttpServer::new(move || {
        App::new()
            .service(
                web::scope("/health")
                    .app_data(app_readiness.clone())
                    .service(health::live)
                    .service(health::ready)
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
