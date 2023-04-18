mod common_handler;
mod note_handler;

use actix_web::web;
use common_handler::common;
use note_handler::{note};

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(common::health_check)
        .service(note::note_list_handler)
        .service(note::create_note_handler)
        .service(note::get_note_handler)
        .service(note::edit_note_handler)
        .service(note::delete_note_handler);

    conf.service(scope);
}
