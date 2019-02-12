use rusqlite::Connection;
use rusqlite::NO_PARAMS;
use rusqlite::MappedRows;
use rusqlite::Row;
use rusqlite::Statement;

struct DB {
    db : Connection
}

impl DB {
    // constructor
    fn new(file : &str) -> DB {
        let mut d = Connection::open(file).unwrap();
        d.execute("create table if not exists u (i unsigned integer)", NO_PARAMS).unwrap();
        DB {db : d}
    }

    // returns length of values
    fn len(&self) -> u32 {
        self.db.query_row("select count(i) from u", NO_PARAMS, |r| r.get(0)).unwrap()
    }

    fn iter<'a, 'stmt: 'a>(&'a self, stmt: &'a mut Statement<'stmt>) -> impl Iterator<Item = rusqlite::Result<u32>> + 'a  {
        let query_map = stmt.query_map(NO_PARAMS, |r| -> u32 {r.get(0)});
        let query_map_unwrapped = query_map.unwrap();
        query_map_unwrapped
    }

    fn print(&self) {
        if self.len() > 0 {
            let mut stmt= self.db.prepare("select i from u").unwrap();
            for i in self.iter(&mut stmt) {
                println!("id {}", i.unwrap());
            }
        } else {
            println!("Nothing");
        }
    }
}

fn main() {

}
