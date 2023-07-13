use actix_web::web;

use super::path::Path;

mod get;
mod save;

pub fn evidence_factory(app: &mut web::ServiceConfig) {
    let base_path = Path{prefix: String::from("/evidence"), backend: true};
    // TODO: probably getter doesn't need and 
    // that one is gets all evidence that is around 1 mln
    app.route(&base_path.prefix, web::get().to(get::get));
    app.route(&base_path.prefix, web::post().to(save::save_evidence));
}
