use dotenv::dotenv;
use std::env;

use mysql::*;

pub fn get_pool() -> Pool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let connection_opts = Opts::from_url(&database_url).unwrap();
    let pool = Pool::new(connection_opts).unwrap();
    return pool;
}





