#[derive(Queryable)]
pub struct User {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub patronymic: String,
    pub email: String,
    pub password: String,
}
