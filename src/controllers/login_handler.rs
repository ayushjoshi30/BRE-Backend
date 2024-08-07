use crate::auth::auth::*;
use crate::error;
use crate::error::Error::WrongCredentialsError;
use crate::models::login_model::{LoginRequest, LoginResponse, Users};
use warp::{reject, reply, Rejection, Reply};
pub type WebResult<T> = std::result::Result<T, Rejection>;
pub type Result<T> = std::result::Result<T, error::Error>;

pub async fn login_handler(users: Users, body: LoginRequest) -> WebResult<impl Reply> {
    match users
        .iter()
        .find(|(_uid, user)| user.email == body.email && user.pw == body.pw)
    {
        Some((uid, user)) => {
            let token =
                create_jwt(&uid, &Role::from_str(&user.role)).map_err(|e| reject::custom(e))?;
            Ok(reply::json(&LoginResponse { token }))
        }
        None => Err(reject::custom(WrongCredentialsError)),
    }
}
<<<<<<< HEAD

pub async fn user_handler(authenticated:bool, db_pool: Arc<DatabaseConnection>) -> WebResult<impl Reply> {
    // Determine the role based on the uid
    if authenticated {
        print!("Authenticating")
    }
    // Call the user data entry function
    let admin_users = UserEntity::find()
        .filter(UserColumn::Role.eq("User")) // Use the correct column variant
        .all(&*db_pool)
        .await
        .map_err(|e| {
            eprintln!("Error retrieving  users: {:?}", e);
            warp::reject::custom(error::Error::DatabaseErrorr)
        })?;

    // Extract the names of the admin users
    let names: Vec<String> = admin_users.into_iter().map(|user| user.username).collect();

    // Create the response object
    let response_obj = json!({
        "message": "Hello user!",
        "user_names": names
    });

    Ok(warp::reply::json(&response_obj))
}

pub async fn admin_handler(authenticated:bool, db_pool: Arc<DatabaseConnection>) -> WebResult<impl Reply> {
    // Return combined responses as a JSON object
    let admin_users = UserEntity::find()
        .filter(UserColumn::Role.eq("Admin")) // Use the correct column variant
        .all(&*db_pool)
        .await
        .map_err(|e| {
            eprintln!("Error retrieving admin users: {:?}", e);
            warp::reject::custom(error::Error::DatabaseErrorr)
        })?;

    // Extract the names of the admin users
    let names: Vec<String> = admin_users.into_iter().map(|user| user.username).collect();

    // Create the response object
    let response_obj = json!({
        "message": "Hello admin!",
        "admin_names": names
    });

    Ok(warp::reply::json(&response_obj))
}
=======
>>>>>>> 284f27156b2dc14bb957816b961734c0b74e10bf
