use rusqlite::Connection;
use uuid::{Uuid};

fn main() {
    println!("learnsqlite!");
    let conn = Connection::open("./rustsqlite.db").expect("open db failed");
    conn.execute(
        "CREATE TABLE IF NOT EXISTS user (
        id    INTEGER PRIMARY KEY,
        name  VARCHAR(50) NOT NULL,
        password  VARCHAR(50) NOT NULL)",
        (),
    )
    .expect("create table fialed");
    conn.execute(
        "insert into user (name,password) values (?1,?2)",
        (format!("{}-{}","hwholiday", "test"),Uuid::new_v4().to_string()),
    )
    .expect("insert into failed");

    let mut resullt = conn.prepare("select * from user;").expect("read failed");
    let data_arr = resullt.query_map((), |row|{
        Ok(User{ id: row.get(0)?, name: row.get(1)?, password: row.get(2)? })
    }).expect("read data failed");
    for item in data_arr{
        println!("{:?}",item.unwrap().to_string());
    }

}

#[derive(Debug)]
struct User {
    id: i64,
    name: String,
    password: String,
}

impl ToString for User {
    fn to_string(&self) -> String {
        format!(
            "id:{},name:{},password:{}",
            self.id, self.name, self.password
        )
    }
}
