use crate::setting::Setting;
use mysql::{prelude::Queryable, Pool, PooledConn};

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
        let pool = Pool::new(Self::url().as_str()).unwrap();
        DB.conn = Some(pool.get_conn().unwrap());
    }

    fn url() -> String {
        let user = dotenv!("DB_USER");
        let password = dotenv!("DB_PASS");
        let url = dotenv!("DB_URL");
        let port = dotenv!("DB_PORT");
        let name = dotenv!("DB_NAME");

        format!("mysql://{user}:{password}@{url}:{port}/{name}")
    }
}

pub fn settings(_id: u64) -> Option<Setting> {
    DB.conn
        .unwrap()
        .query_map(
            "SELECT * FROM settings WHERE guild_id = {_id}",
            |mute_role_id| Setting { mute_role_id },
        )
        .ok()?
        .pop()
}
