use rusqlite::{params, Connection, Result};

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>,
}
fn yes()->Result<()>{
    Ok(())
}
fn main() -> Result<()> {
    let conn = Connection::open_in_memory()?;

    conn.execute(
        "CREATE TABLE person (
                  id              INTEGER PRIMARY KEY,
                  name            TEXT NOT NULL,
                  data            BLOB
                  )",
        [],
    )?;



    let me = Person {
        id: 0,
        name: "Stev111111en".to_string(),
        data: None,
    };

    conn.execute(
        "INSERT INTO person (name, data) VALUES (?1, ?2)",
        params![me.name, me.data],
    )?;

    let mut stmt = conn.prepare("SELECT id, name, data FROM person")?;
    let person_iterator = stmt.query_map([], |row| {
        return Ok(Person {

            id: row.get(0)?,
            name: row.get(1)?,
            data: row.get(2)?,
        });
        
    })?;

    for person in person_iterator {
        println!("Found person {:?}", person.unwrap());
    }
    Ok(())
}