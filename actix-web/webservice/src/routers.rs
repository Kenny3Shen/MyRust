use crate::handlers::general::*;
use crate::handlers::course::*;
use crate::handlers::teacher::*;
use actix_web::web;

pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check_handler));
}

// 定义 courses 的作用域
pub fn course_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/courses")
            .route("/", web::post().to(post_new_course))
            .route("/{teacher_id}", web::get().to(get_courses_for_teacher))
            .route("/{teacher_id}/{course_id}", web::get().to(get_course_detail))
            .route("/{teacher_id}/{course_id}", web::delete().to(delete_course))
            .route("/{teacher_id}/{course_id}", web::put().to(update_course_details))
    );
}

pub fn teacher_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/teachers")
        .route("/add_teacher", web::post().to(post_new_teacher))
        .route("/get_all_teachers", web::get().to(get_all_teachers))
        .route("/get_teacher_{teacher_id}", web::get().to(get_teacher_details))
        .route("/update_teacher_{teacher_id}", web::put().to(update_teacher_details))
        .route("/delete_teacher_{teacher_id}", web::delete().to(delete_teacher))
    );
}