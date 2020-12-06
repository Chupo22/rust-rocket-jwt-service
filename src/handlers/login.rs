// use crate::db;
use crate::models::User;
use crate::schema::auth::user;
use bcrypt::verify;
use diesel;
use diesel::ExpressionMethods;
use diesel::PgConnection;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use user::dsl::email;

pub struct LoginRequest {
    pub login: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    access_token: String,
    refresh_token: String,
}

impl LoginRequest {
    pub fn new(login: String, password: String) -> LoginRequest {
        LoginRequest { login, password }
    }
}

pub fn login(args: LoginRequest, connection: &PgConnection) -> Result<LoginResponse, String> {
    let users = user::table
        .limit(1)
        .filter(email.eq(&args.login))
        .load::<User>(connection)
        .map_err(|e| e.to_string())?;

    let hash = match users.first() {
        Some(user) => Ok(&user.password),
        None => Err("User not found".to_string()),
    }?;

    match verify(&args.password, hash) {
        Ok(true) => Ok(LoginResponse {
            // TODO: Implement me
            access_token: "TMP".to_string(),
            refresh_token: "TMP".to_string(),
        }),
        Ok(false) => Err("Invalid password".to_string()),
        Err(err) => Err(err.to_string()),
    }
}
