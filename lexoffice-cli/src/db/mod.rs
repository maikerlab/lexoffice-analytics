use std::env;

use diesel::{Connection, PgConnection};

use self::models::Voucher;

pub mod models;
pub mod schema;

use diesel::prelude::*;

pub fn connect_db() -> PgConnection {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgConnection::establish(&db_url).unwrap_or_else(|_| panic!("Error connecting to {}", db_url))
}

pub fn get_all_vouchers() -> Vec<Voucher> {
    use self::schema::vouchers::dsl::*;
    let conn = &mut connect_db();

    let results = vouchers.select(Voucher::as_select()).load(conn);

    match results {
        Ok(res) => res,
        Err(e) => {
            println!("Error getting vouchers: {}", e);
            vec![]
        }
    }
}
