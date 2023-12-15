use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Password {
    pub id: Option<u32>,             // 数据库自增id，插入时不需要
    pub name: String,                // 网站或服务名称
    pub url: Option<String>,         // 链接，如果服务不是网站则为null
    pub description: Option<String>, // 描述信息
    pub username: String,            // 登录用户名
    pub email: Option<String>,       // 邮箱
    pub phone: Option<String>,       // 手机号
    pub pwd: String,                 // 密码
    pub last_update: Option<String>, // 最后更新时间
}

#[tauri::command]
pub fn init_db() -> rusqlite::Result<usize> {
    let connection = Connection::open("data.db")?;
    connection.execute(
        "CREATE TABLE IF NOT EXISTS my_password (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            url TEXT,
            description TEXT,
            username TEXT NOT NULL,
            email TEXT,
            phone TEXT,
            pwd TEXT NOT NULL,
            last_update TEXT DEFAULT (datetime('now','localtime'))
        )",
        [],
    )
}

pub fn insert_data(password: &Password) -> rusqlite::Result<usize> {
    let conn = Connection::open("data.db")?;
    conn.execute(
        "INSERT INTO my_password (
            name,
            url,
            description,
            username,
            email,
            phone,
            pwd
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![
            password.name,
            password.url,
            password.description,
            password.username,
            password.email,
            password.phone,
            password.pwd
        ],
    )
}

pub fn query_data(query_content: String) -> rusqlite::Result<Vec<Password>> {
    let conn = Connection::open("data.db")?;
    let mut stmt = conn.prepare(
        "SELECT * FROM my_password
        WHERE name LIKE '%' || :keyword || '%' COLLATE NOCASE
        OR description LIKE '%' || :keyword || '%' COLLATE NOCASE
        OR url LIKE '%' || :keyword || '%' COLLATE NOCASE",
    )?;
    let rows = stmt.query_map(&[(":keyword", &query_content)], |row| {
        Ok(Password {
            id: row.get(0)?,
            name: row.get(1)?,
            url: row.get(2)?,
            description: row.get(3)?,
            username: row.get(4)?,
            email: row.get(5)?,
            phone: row.get(6)?,
            pwd: row.get(7)?,
            last_update: row.get(8)?,
        })
    });
    let res = rows?.collect();
    res
}

pub fn query_data_by_id(id: u32) -> rusqlite::Result<Password> {
    let conn = Connection::open("data.db")?;
    let mut stmt = conn.prepare(
        "SELECT * FROM my_password
        WHERE id = ?1",
    )?;
    stmt.query_row([id], |row| {
        Ok(Password {
            id: row.get(0)?,
            name: row.get(1)?,
            url: row.get(2)?,
            description: row.get(3)?,
            username: row.get(4)?,
            email: row.get(5)?,
            phone: row.get(6)?,
            pwd: row.get(7)?,
            last_update: row.get(8)?,
        })
    })
}

pub fn edit_data_by_id(id: u32, password: &Password) -> rusqlite::Result<usize> {
    let conn = Connection::open("data.db")?;
    conn.execute(
        "UPDATE my_password SET
            name = ?1,
            url = ?2,
            description = ?3,
            username = ?4,
            email = ?5,
            phone = ?6,
            pwd = ?7
        WHERE id = ?8",
        params![
            password.name,
            password.url,
            password.description,
            password.username,
            password.email,
            password.phone,
            password.pwd,
            id
        ],
    )
}

pub fn delete_data(id: u32) -> rusqlite::Result<usize> {
    let conn = Connection::open("data.db")?;
    conn.execute("DELETE FROM my_password WHERE id = ?1", params![id])
}
