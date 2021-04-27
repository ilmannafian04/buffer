use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};

pub type DbPool = Pool<ConnectionManager<PgConnection>>;
