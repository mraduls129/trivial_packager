// register_handler.rs
use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{models::{Pool, User}, auth_handler, error};
use crate::util::hash_password;
// UserData is used to extract data from a post request by the client
#[derive(Debug, Deserialize, Serialize)]
pub struct UserData {
    pub name: String,
    pub email: String,
    pub gender: String,
    pub courses: Vec<String>,
}

pub async fn register_user(
    user_data: web::Json<UserData>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_data = user_data.into_inner();

    if !(user_data.gender == *"B" || user_data.gender == *"G") {
        return Err(actix_web::error::ErrorBadRequest("Missing gender"));
    }

    web::block(move || create_user_query(user_data, pool)).await??;

    Ok(HttpResponse::Ok().finish())
}

pub async fn flush_users(logged_user: auth_handler::AuthToken, pool: web::Data<Pool>) -> Result<HttpResponse, actix_web::Error> {
    if let auth_handler::AuthToken::User(user) = logged_user{
        if user.role == "A" {
            web::block(move || flush_users_query(pool)).await??;
            return Ok(HttpResponse::Ok().finish());
        }
    }
    
    Err(error::ServiceError::Unauthorized.into())
}

fn create_user_query(user_data: UserData, pool: web::Data<Pool>) -> Result<(), error::ServiceError> {
    use crate::schema::users::dsl::users;

    let role = "T".to_string();

    let mut conn = pool.get()?;
    let password: String = hash_password("1777")?;
    let user = User::from(
        &user_data.name,
        &user_data.email.clone(),
        &password,
        &user_data.gender,
        &role,
    );
    let email = user_data.email;
    match diesel::insert_into(users).values(&user).execute(&mut conn) {
        Ok(_) => {
            use crate::schema::courses::dsl::*;

            for course in user_data.courses {
                let mut course = course.split("-"); 
                let level = course.next().unwrap();
                let group = course.next().unwrap();
                let subject = course.next().unwrap();

                if subject == "anatomia" {
                    diesel::update(courses)
                    .filter(id.eq(level.to_owned() + "-" + group))
                    .set(anatomia.eq(email.clone()))
                    .execute(&mut conn).unwrap();
                }
                else if subject == "english" {
                    diesel::update(courses)
                    .filter(id.eq(level.to_owned() + "-" + group))
                    .set(english.eq(email.clone()))
                    .execute(&mut conn).unwrap();
                }
                else if subject == "biologia" {
                    diesel::update(courses)
                    .filter(id.eq(level.to_owned() + "-" + group))
                    .set(biologia.eq(email.clone()))
                    .execute(&mut conn).unwrap();                }
                else if subject == "castellano" {
                    diesel::update(courses)
                    .filter(id.eq(level.to_owned() + "-" + group))
                    .set(castellano.eq(email.clone()))
                    .execute(&mut conn).unwrap();                }
                else if subject == "clasica" {
                    diesel::update(courses)
                    .filter(id.eq(level.to_owned() + "-" + group))
                    .set(clasica.eq(email.clone()))
                    .execute(&mut conn).unwrap();                }
                else if subject == "dibuix" {
                    diesel::update(courses)
                    .filter(id.eq(level.to_owned() + "-" + group))
                    .set(dibuix.eq(email.clone()))
                    .execute(&mut conn).unwrap();                }else if subject == "ed_fisica" {
                    diesel::update(courses)
                    .filter(id.eq(level.to_owned() + "-" + group))
                    .set(ed_fisica.eq(email.clone()))
                    .execute(&mut conn).unwrap();                }
                else if subject == "filosofia" {
                    diesel::update(courses)
                    .filter(id.eq(level.to_owned() + "-" + group))
                    .set(filosofia.eq(email.clone()))
                    .execute(&mut conn).unwrap();                }
                else if subject == "fisica_quimica" {
                    diesel::update(courses)
                    .filter(id.eq(level.to_owned() + "-" + group))
                    .set(fisica_quimica.eq(email.clone()))
                    .execute(&mut conn).unwrap();                }
                else if subject == "frances" {
                    diesel::update(courses)
                    .filter(id.eq(level.to_owned() + "-" + group))
                    .set(frances.eq(email.clone()))
                    .execute(&mut conn).unwrap();                }
                else if subject == "historia" {
                    diesel::update(courses)
                    .filter(id.eq(level.to_owned() + "-" + group))
                    .set(historia.eq(email.clone()))
                    .execute(&mut conn).unwrap();                }
                else if subject == "grec" {
                    diesel::update(courses)
                    .filter(id.eq(level.to_owned() + "-" + group))
                    .set(grec.eq(email.clone()))
                    .execute(&mut conn).unwrap();                }
                else if subject == "informatica" {
                    diesel::update(courses)
                    .filter(id.eq(level.to_owned() + "-" + group))
                    .set(informatica.eq(email.clone()))
                    .execute(&mut conn).unwrap();                }
                else if subject == "literatura" {
                    diesel::update(courses)
                    .filter(id.eq(level.to_owned() + "-" + group))
                    .set(literatura.eq(email.clone()))
                    .execute(&mut conn).unwrap();                }
                else if subject == "llati" {
                    diesel::update(courses)
                    .filter(id.eq(level.to_owned() + "-" + group))
                    .set(llati.eq(email.clone()))
                    .execute(&mut conn).unwrap();                }
                else if subject == "mates" {
                    diesel::update(courses)
                    .filter(id.eq(level.to_owned() + "-" + group))
                    .set(mates.eq(email.clone()))
                    .execute(&mut conn).unwrap();                }
                else if subject == "musica" {
                    diesel::update(courses)
                    .filter(id.eq(level.to_owned() + "-" + group))
                    .set(musica.eq(email.clone()))
                    .execute(&mut conn).unwrap();                }
                else if subject == "orientacio" {
                    diesel::update(courses)
                    .filter(id.eq(level.to_owned() + "-" + group))
                    .set(orientacio.eq(email.clone()))
                    .execute(&mut conn).unwrap();                }
                else if subject == "plastica" {
                    diesel::update(courses)
                    .filter(id.eq(level.to_owned() + "-" + group))
                    .set(plastica.eq(email.clone()))
                    .execute(&mut conn).unwrap();                }
                else if subject == "religio" {
                    diesel::update(courses)
                    .filter(id.eq(level.to_owned() + "-" + group))
                    .set(religio.eq(email.clone()))
                    .execute(&mut conn).unwrap();                }
                else if subject == "tecnologia" {
                    diesel::update(courses)
                    .filter(id.eq(level.to_owned() + "-" + group))
                    .set(tecnologia.eq(email.clone()))
                    .execute(&mut conn).unwrap();                }
                else if subject == "valencia" {
                    diesel::update(courses)
                    .filter(id.eq(level.to_owned() + "-" + group))
                    .set(valencia.eq(email.clone()))
                    .execute(&mut conn).unwrap();                }
                else {
                    diesel::update(courses)
                    .filter(id.eq(level.to_owned() + "-" + group))
                    .set(etica.eq(email.clone()))
                    .execute(&mut conn).unwrap();                }
            }

            Ok(())
        }
        Err(_) => Err(error::ServiceError::InternalServerError),
    }
}

fn flush_users_query(pool: web::Data<Pool>) -> Result<(), crate::error::ServiceError> {
    use crate::schema::users::dsl::*;

    let mut conn = pool.get()?;

    diesel::delete(users).filter(email.ne("asengar2009@gmail.com")).execute(&mut conn)?;

    Ok(())
}