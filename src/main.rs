use actix_files::NamedFile;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{get, http::StatusCode, middleware, web, App, HttpServer, Result};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use std::path::PathBuf;
use trivial_packager::{auth_handler, config::Config, image, models, question, register_handler};

#[get("/favicon.ico")]
async fn publish_favicon() -> Result<NamedFile> {
    let path = PathBuf::from("static/images/favicon.ico");
    Ok(NamedFile::open(path)?)
}

#[get("/images/not-found.png")]
async fn publish_not_found() -> Result<NamedFile> {
    let path = PathBuf::from("static/images/not-found.png");
    Ok(NamedFile::open(path)?)
}

#[get("/")]
async fn publish_index(_logged_user: auth_handler::AuthToken) -> Result<NamedFile> {
    let path = PathBuf::from("static/index.html");
    Ok(NamedFile::open(path)?)
}

#[get("/login")]
async fn publish_login_html() -> Result<NamedFile> {
    let path = PathBuf::from("static/login.html");
    Ok(NamedFile::open(path)?)
}

#[get("/add_question")]
async fn publish_add_question_html(_logged_user: auth_handler::AuthToken) -> Result<NamedFile> {
    let path = PathBuf::from("static/add_question.html");
    Ok(NamedFile::open(path)?)
}

#[get("/modify_question")]
async fn publish_modify_question_html(_logged_user: auth_handler::AuthToken) -> Result<NamedFile> {
    let path = PathBuf::from("static/modify_question.html");
    Ok(NamedFile::open(path)?)
}

#[get("/student_question")]
async fn publish_student_question_html(_logged_user: auth_handler::AuthToken) -> Result<NamedFile> {
    let path = PathBuf::from("static/student_question.html");
    Ok(NamedFile::open(path)?)
}

#[get("/statics")]
async fn publish_statics_html(_logged_user: auth_handler::AuthToken) -> Result<NamedFile> {
    let path = PathBuf::from("static/statics.html");
    Ok(NamedFile::open(path)?)
}

#[get("/register")]
async fn publish_register() -> Result<NamedFile> {
    let path = PathBuf::from("static/register.html");
    Ok(NamedFile::open(path)?)
}


#[get("/static/js/color-modes.js")]
async fn publish_color_modes_js() -> Result<NamedFile> {
    let path = PathBuf::from("static/js/color-modes.js");
    Ok(NamedFile::open(path)?)
}

#[get("/static/css/sign-in.css")]
async fn publish_sign_in_css() -> Result<NamedFile> {
    let path = PathBuf::from("static/css/sign-in.css");
    Ok(NamedFile::open(path)?)
}

#[get("/static/js/sign-in.js")]
async fn publish_sign_in_js() -> Result<NamedFile> {
    let path = PathBuf::from("static/js/sign-in.js");
    Ok(NamedFile::open(path)?)
}

#[get("/static/css/index.css")]
async fn publish_index_css() -> Result<NamedFile> {
    let path = PathBuf::from("static/css/index.css");
    Ok(NamedFile::open(path)?)
}

#[get("/static/css/add_question.css")]
async fn publish_add_question_css() -> Result<NamedFile> {
    let path = PathBuf::from("static/css/add_question.css");
    Ok(NamedFile::open(path)?)
}

#[get("/static/js/index.js")]
async fn publish_index_js() -> Result<NamedFile> {
    let path = PathBuf::from("static/js/index.js");
    Ok(NamedFile::open(path)?)
}

#[get("/static/js/add_question.js")]
async fn publish_add_question_js() -> Result<NamedFile> {
    let path = PathBuf::from("static/js/add_question.js");
    Ok(NamedFile::open(path)?)
}

#[get("/static/js/modify_question.js")]
async fn publish_modify_question_js() -> Result<NamedFile> {
    let path = PathBuf::from("static/js/modify_question.js");
    Ok(NamedFile::open(path)?)
}

#[get("/static/js/student_question.js")]
async fn publish_student_question_js() -> Result<NamedFile> {
    let path = PathBuf::from("static/js/student_question.js");
    Ok(NamedFile::open(path)?)
}

#[get("/static/js/statics.js")]
async fn publish_statics_js() -> Result<NamedFile> {
    let path = PathBuf::from("static/js/statics.js");
    Ok(NamedFile::open(path)?)
}

#[get("/static/js/register.js")]
async fn publish_register_js() -> Result<NamedFile> {
    let path: PathBuf = PathBuf::from("static/js/register.js");
    Ok(NamedFile::open(path)?)
}

async fn not_found() -> Result<NamedFile> {
    let path = PathBuf::from("static/404.html");
    #[allow(deprecated)]
    Ok(NamedFile::open(path)?.set_status_code(StatusCode::NOT_FOUND))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::new("./config.json");
    let config_copy = Config::new("./config.json");

    dotenv::dotenv().ok();
    std::env::set_var(
        "RUST_LOG",
        "simple-auth-server=debug,actix_web=info,actix_server=info",
    );
    env_logger::init();

    // create db connection pool
    let manager = ConnectionManager::<MysqlConnection>::new(config.db_url);
    let pool: models::Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    // Start http server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            // enable logger
            .wrap(middleware::Logger::default())
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&config.cookie_key)
                    .name("auth")
                    .path("/")
                    .domain(config.domain.clone())
                    .max_age(time::Duration::days(1))
                    .secure(false), // this can only be true if you have https
            ))
            // limit the maximum amount of data that server will accept
            .app_data(web::JsonConfig::default().limit(4096))
            // everything under '/api/' route
            .service(
                web::scope("/api")
                    .service(
                        web::resource("/add_user")
                            .route(web::post().to(register_handler::register_user))
                            .route(web::get().to(register_handler::flush_users)),
                    )
                    .service(
                        web::resource("/auth")
                            .route(web::post().to(auth_handler::login))
                            .route(web::delete().to(auth_handler::logout))
                            .route(web::get().to(auth_handler::get_me))
                            .route(web::patch().to(auth_handler::modify_password)),
                    )
                    .service(
                        web::resource("/question")
                            .route(web::post().to(question::new_question))
                            .route(web::delete().to(question::delete_question))
                            .route(web::patch().to(question::verify_question)),
                    )
                    .service(
                        web::resource("/filter_question")
                            .route(web::post().to(question::get_questions)),
                    )
                    .service(web::resource("/image").route(web::post().to(image::save_file))),
            )
            .service(publish_favicon)
            .service(publish_index)
            .service(publish_login_html)
            .service(publish_add_question_html)
            .service(publish_modify_question_html)
            .service(publish_color_modes_js)
            .service(publish_sign_in_css)
            .service(publish_sign_in_js)
            .service(publish_index_css)
            .service(publish_add_question_css)
            .service(publish_index_js)
            .service(publish_add_question_js)
            .service(publish_modify_question_js)
            .service(publish_student_question_html)
            .service(publish_student_question_js)
            .service(publish_not_found)
            .service(publish_statics_html)
            .service(publish_statics_js)
            .service(publish_register)
            .service(publish_register_js)
            .service(actix_files::Files::new("/images", "images").prefer_utf8(true))
            .default_service(web::route().to(not_found))
    })
    .bind(format!("{}:{}", config_copy.domain, config_copy.port))?
    .run()
    .await
}
