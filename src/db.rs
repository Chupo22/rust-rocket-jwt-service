use diesel::PgConnection;
use dotenv;
use r2d2;
use r2d2_diesel::ConnectionManager;
use rocket::http::Status;
use rocket::request::FromRequest;
use rocket::{request, Outcome, Request, State};
use rocket_contrib::databases::diesel;
use std::ops::Deref;

type PgManager = ConnectionManager<PgConnection>;
type PgPool = r2d2::Pool<PgManager>;

fn database_url() -> String {
    dotenv::var("DATABASE_URL").expect("DATABASE_URL env must be set")
}

pub struct Connection(pub r2d2::PooledConnection<PgManager>);

impl<'a, 'r> FromRequest<'a, 'r> for Connection {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let pool = request.guard::<State<PgPool>>()?;

        match pool.get() {
            Ok(conn) => Outcome::Success(Connection(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

// For the convenience of using an &Connection as an &PgConnection.
impl Deref for Connection {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn init_pool() -> PgPool {
    let manager = PgManager::new(database_url());

    PgPool::new(manager).expect("db pool")
}
