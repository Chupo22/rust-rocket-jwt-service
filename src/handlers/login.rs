// use crate::db;
use crate::models::User;
use crate::schema::auth::user;
use bcrypt::verify;
use diesel::ExpressionMethods;
use diesel::PgConnection;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use user::dsl::email;
use diesel;

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
    let password = &args.password;
    let response = user::table
        .limit(1)
        .filter(email.eq(args.login))
        .load::<User>(connection)
        .and_then(|users| match users.into_iter().next() {
            Some(user) => Ok(user),
            None => Err(diesel::result::Error::NotFound)
        })
        .map_err(|_| "User not found".to_string())
        .map(|user| verify(password, &user.password))
        .and_then(|check_result| match check_result {
            Ok(true) => Ok(LoginResponse {
                // TODO: Implement me
                access_token: "TMP".to_string(),
                refresh_token: "TMP".to_string(),
            }),
            Ok(false) => Err("Invalid password".to_string()),
            Err(err) => Err(err.to_string())
        });

    response
}
