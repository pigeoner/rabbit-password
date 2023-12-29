use aes::cipher::{generic_array::GenericArray, BlockDecrypt, BlockEncrypt, KeyInit};
use aes::Aes256;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::str::from_utf8;

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

pub fn update_all_pwd(new_pwd: String, old_pwd: String) -> rusqlite::Result<usize> {
    let conn = Connection::open("data.db")?;
    // 首先查询数据库中所有数据的密码字段和id字段
    let mut stmt = conn.prepare("SELECT id, pwd FROM my_password")?;
    let rows = stmt.query_map(
        [],
        |row: &rusqlite::Row| -> rusqlite::Result<(u32, String)> { Ok((row.get(0)?, row.get(1)?)) },
    )?;
    let res: Vec<(u32, String)> = vec![rows.collect::<rusqlite::Result<Vec<_>>>()?]
        .into_iter()
        .flatten()
        .collect();

    // 对每一条数据的密码进行解密
    let mut pwd_data = Vec::new();
    for (id, pwd) in res {
        let cipher = Aes256::new(GenericArray::from_slice(&old_pwd.as_bytes()[..32]));
        let mut decrypted_pwd = GenericArray::clone_from_slice(&pwd.as_bytes());
        println!("fdsagrlkjidsvbh");
        cipher.decrypt_block(&mut decrypted_pwd);
        pwd_data.push((id, decrypted_pwd));
    }
    println!("解密");

    // 对每一条数据的密码进行加密
    for (id, mut pwd) in &mut pwd_data {
        let cipher = Aes256::new(GenericArray::from_slice(new_pwd.as_bytes()));
        cipher.encrypt_block(&mut pwd);
    }
    println!("加密");

    // 更新数据库中的密码字段
    // conn.execute(
    //     "UPDATE my_password SET pwd = ?1 WHERE id = ?2",
    //     params![encrypted_data[0].1, encrypted_data[0].0],
    // )
    println!("{:?}", pwd_data);
    conn.execute("SELECT * FROM my_password", [])
}
