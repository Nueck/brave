use crate::config::GLOBAL_ENV_CONFIG;
use sea_orm::{Database, DatabaseConnection};

pub async fn connect_db() -> DatabaseConnection {
    let opt = GLOBAL_ENV_CONFIG.pg.get_pb_connect_opt();

    let conn = match opt {
        Ok(opt) => {
            //连接数据库
            Database::connect(opt).await
        }
        Err(err) => {
            log::error!("{}", err.to_string());
            panic!();
        }
    };

    let connect = match conn {
        Ok(conn) => {
            log::info!("Database connection successful");
            conn
        }
        Err(err) => {
            log::error!("Database connection failure: {}", err.to_string());
            panic!();
        }
    };
    connect
}
