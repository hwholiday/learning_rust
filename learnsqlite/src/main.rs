use rusqlite::Connection;
use uuid::Uuid;

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
        (
            format!("{}-{}", "hwholiday", "test"),
            Uuid::new_v4().to_string(),
        ),
    )
    .expect("insert into failed");

    let mut result = conn.prepare("select * from user;").expect("read failed");
    let data_arr = result
        .query_map((), |row| {
            Ok(User {
                id: row.get(0)?,
                name: row.get(1)?,
                password: row.get(2)?,
            })
        })
        .expect("read data failed");
    for item in data_arr {
        println!("query_map {:?}", item.unwrap().to_string());
    }
    let mut result_single = conn.prepare("select * from user where id = 1;").unwrap();
    let result_single_rows = result_single.query_row((), |row| {
        Ok(User {
            id: row.get(0)?,
            name: row.get(1)?,
            password: row.get(2)?,
        })
    });
    println!("query_row {:?}", result_single_rows.unwrap().to_string());
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
