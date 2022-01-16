use super::AppState;
use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(hello);
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

// #[get("/user/{id}")]
// async fn get_user(
//     user_id: web::Path<String>,
//     app_state: web::Data<AppState<'_>>,
// ) -> impl Responder {
//     let user = app_state.context.users.get_user_by_id(&user_id).await;

//     match user {
//         Err(_) => HttpResponse::NotFound().finish(),
//         Ok(mut user) => {
//             let groups = app_state
//                 .context
//                 .users_to_groups
//                 .get_groups_by_user_id(&user.id)
//                 .await;

//             match groups {
//                 Err(_) => HttpResponse::InternalServerError().finish(),
//                 Ok(groups) => {
//                     user.groups = groups;
//                     HttpResponse::Ok().json(user)
//                 }
//             }
//         }
//     }
// }