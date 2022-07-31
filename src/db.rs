use crate::setting::Setting;
use mysql::{Pool, PooledConn};

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
        String::new()
    }

    pub fn get_settings(&mut self) -> Option<Setting> {
        None
    }
}
