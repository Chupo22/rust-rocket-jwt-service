use uuid::Uuid;

#[derive(Queryable)]
pub struct RefreshToken {
    pub id: String,
    pub user_id: String,
    pub toke: String,
}

#[derive(Serialize, Queryable)]
pub struct User {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub patronymic: String,
    pub email: String,
    pub password: String,
}
