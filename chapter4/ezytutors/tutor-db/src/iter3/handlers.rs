use actix_web::{HttpResponse, web};

use crate::{
    db_access::{get_course_details_db, get_courses_for_tutor_db, post_new_course_db},
    models::Course,
    state::AppState,
};

pub async fn health_check_handler(app_state: web::Data<AppState>) -> HttpResponse {
    let health_check_reponse = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();
    let response = format!("{} {} times", health_check_reponse, visit_count);
    *visit_count += 1;
    HttpResponse::Ok().json(&response)
}

pub async fn get_courses_for_tutor(
    app_state: web::Data<AppState>,
    params: web::Path<(i32,)>,
) -> HttpResponse {
    let tuple = params.0;
    let tutor_id: i32 = tuple;
    let courses = get_courses_for_tutor_db(&app_state.db, tutor_id).await;
    HttpResponse::Ok().json(courses)
}

pub async fn get_course_details(
    app_state: web::Data<AppState>,
    params: web::Path<(i32, i32)>,
) -> HttpResponse {
    let (tutor_id, course_id) = (params.0, params.1);
    let course = get_course_details_db(&app_state.db, tutor_id, course_id).await;
    HttpResponse::Ok().json(course)
}

/*  复制粘贴命令
curl -X POST localhost:3000/courses/ \
-H "Content-Type: application/json" \
 -d '{"tutor_id":1, "course_name":"Hello there customer 1 !"}'
*/
pub async fn post_new_course(
    new_course: web::Json<Course>,
    app_state: web::Data<AppState>,
) -> HttpResponse {
    let course = post_new_course_db(&app_state.db, new_course.into()).await;
    HttpResponse::Ok().json(course)
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;
    use chrono::NaiveDate;
    use dotenv::dotenv;
    use sqlx::postgres::PgPool;
    use std::env;
    use std::sync::Mutex;

    #[actix_rt::test]
    async fn get_all_courses_success() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let pool: PgPool = PgPool::connect(&database_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: pool,
        });
        let tutor_id: web::Path<(i32,)> = web::Path::from((1,));
        let resp = get_courses_for_tutor(app_state, tutor_id).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_course_detail_test() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let pool: PgPool = PgPool::connect(&database_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: pool,
        });
        let params: web::Path<(i32, i32)> = web::Path::from((1, 2));
        let resp = get_course_details(app_state, params).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn post_course_success() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let pool: PgPool = PgPool::connect(&database_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: pool,
        });

        let new_course_msg = Course {
            course_id: 3,
            tutor_id: 1,
            course_name: "This is the next course".into(),
            posted_time: Some(
                NaiveDate::from_ymd_opt(2020, 9, 17)
                    .unwrap()
                    .and_hms_opt(14, 01, 11)
                    .unwrap(),
            ),
        };

        let course_param = web::Json(new_course_msg);
        let resp = post_new_course(course_param, app_state).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
