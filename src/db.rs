use std::env;

use crate::setting::Setting;
use mysql::{Pool, PooledConn};

static mut DB: Box<DataBase> = Box::new(DataBase::new());

pub struct DataBase {
    pool: Pool,
    conn: PooledConn,
}

impl DataBase {
    pub fn new() -> Self {
        let pool = Pool::new(Self::url().as_str()).unwrap();
        let conn = pool.get_conn().unwrap();

        Self { pool, conn }
    }

    fn url() -> String {
        let user = env::var("DB_USER").unwrap();
        let password = env::var("DB_PASS").unwrap();
        let name = env::var("DB_NAME").unwrap();
        let url = env::var("DB_URL").unwrap();
        let port = env::var("DB_PORT").unwrap();

        format!("mysql://{}:{}@{}:{}/{}", user, password, url, port, name)
    }
}

pub fn settings(id: u64) -> Option<Setting> {
    None
}
