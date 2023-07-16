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
        web::scope("/api/courses")
            .route("/add_course", web::post().to(post_new_course))
            .route("/get_all_courses_{teacher_id}", web::get().to(get_courses_for_teacher))
            .route("/get_course_{teacher_id}_{course_id}", web::get().to(get_course_detail))
            .route("/del_course_{teacher_id}_{course_id}", web::delete().to(delete_course))
            .route("/up_course_{teacher_id}_{course_id}", web::put().to(update_course_details))
    );
}

pub fn teacher_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/teachers")
        .route("/add_teacher", web::post().to(post_new_teacher))
        .route("/get_all_teachers", web::get().to(get_all_teachers))
        .route("/get_teacher/tid={teacher_id}", web::get().to(get_teacher_details))
        .route("/up_teacher_{teacher_id}", web::put().to(update_teacher_details))
        .route("/del_teacher_{teacher_id}", web::delete().to(delete_teacher))
    );
}