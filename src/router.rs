use actix_web::Scope;

use crate::services::gitea::route::scoped_config as gitea_service;

/// Init service routes
pub fn init_api_service(scope: Scope) -> Scope {
    scope.configure(gitea_service)
}
