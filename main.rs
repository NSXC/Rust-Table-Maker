#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]
use mysql::*;
use mysql::prelude::*;
use chrono::prelude::*;
use bcrypt::{hash, verify, DEFAULT_COST};
fn main() {
    let url = "mysql://root:password@localhost:3306/users";
    let pool = Pool::new(url).unwrap();
    let mut conn = pool.get_conn().unwrap();
    let table_exists = conn.query_first::<u8, _>("SELECT COUNT(*) FROM information_schema.tables WHERE table_schema = 'DATABASENAME' AND table_name = 'TABLENAME'")
        .unwrap()
        .unwrap_or(0) > 0;
    if !table_exists {
        conn.query_drop(
            "CREATE TABLE NAME (
               //TABLE INFO HERE
            )"
        ).unwrap();
    }
}
    
