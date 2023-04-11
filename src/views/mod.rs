mod auth;
mod to_do; // define the module
mod app;

use actix_web::web::ServiceConfig;

use auth::auth_views_factory;
use to_do::to_do_views_factory; // import the factory
use app::app_views_factory;

pub fn views_factory(app: &mut ServiceConfig) {
    auth_views_factory(app);
    to_do_views_factory(app); // pass the ServiceConfig
    app_views_factory(app);
}