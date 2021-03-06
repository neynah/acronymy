#![crate_name="initdb"]
#![crate_type = "bin"]

extern crate sqlite3;

mod init {
    use sqlite3::{open, Database, SqliteResult};
    use std::io::BufRead;

    pub fn write_db(db : &mut Database) -> SqliteResult<()> {
        try!(db.exec("CREATE TABLE Words(Word TEXT);"));
        try!(db.exec("CREATE TABLE Definitions(Definee TEXT, Idx INTEGER, Definer TEXT);"));
        try!(db.exec("CREATE TABLE Log(Word TEXT, Timestamp INTEGER);"));

        let input = ::std::io::stdin();
        for line in input.lock().lines() {
            let word = line.unwrap().clone();
            let trimmed = word.trim();
            assert!(trimmed.chars().all(|c| c.is_alphanumeric()), "not alphanumeric: {}", trimmed);
            try!(db.exec(&format!("INSERT INTO Words VALUES(\"{}\");", trimmed)));
        }
        Ok(())
    }

    pub fn open_db() -> SqliteResult<Database> {
        let args : Vec<String> = ::std::env::args().collect();
        return open(&args[1]);
    }

    pub fn main() {
        match open_db() {
            Ok(mut db) => {
               match write_db(&mut db) {
                   Ok(()) => {}
                   Err(e) => { println!("error: {:?}, ({})", e, db.get_errmsg()) }
               }
            }
            Err(e) => {
               println!("could not open database: {:?}", e);
            }
        }
    }
}

pub fn main() {
    init::main();
}
