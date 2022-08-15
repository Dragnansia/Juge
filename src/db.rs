use crate::setting::Setting;
use mysql::{prelude::Queryable, Pool, PooledConn};
use std::env;

const DB: DataBase = DataBase {
    pool: None,
    conn: None,
};

pub struct DataBase {
    pub pool: Option<Pool>,
    pub conn: Option<PooledConn>,
}

impl DataBase {
    pub fn init() {
        DB.pool = Some(Pool::new(Self::url().as_str()).unwrap());
        DB.conn = Some(DB.pool.unwrap().get_conn().unwrap());
    }

    fn url() -> String {
        let user = env::var("DB_USER").unwrap_or(String::from("root"));
        let password = env::var("DB_PASS").unwrap_or(String::from("root"));
        let url = env::var("DB_URL").unwrap_or(String::from("localhost"));
        let port = env::var("DB_PORT").unwrap_or(String::from("3306"));
        let name = env::var("DB_NAME").unwrap_or(String::from("juge"));

        format!("mysql://{user}:{password}@{url}:{port}/{name}")
    }
}

pub fn settings(id: u64) -> Option<Setting> {
    DB.conn
        .unwrap()
        .query_map("SELECT * FROM settings WHERE guild_id = {id}", |mute_role_id| {
            Setting { mute_role_id }
        })
        .ok()?
        .pop()
}
