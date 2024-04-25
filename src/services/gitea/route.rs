use actix_web::web;

use crate::services::gitea::handler::test;

pub fn scoped_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/webhook").route("/test", web::post().to(test)), // .route("/main", web::get().to(main))
    );
}
