use actix::prelude::*;
use std::io;
use diesel;
use diesel::prelude::*;
use diesel::result::Error;

pub struct DbExecutor {
    conn: PgConnection,
}

unsafe impl Send for DbExecutor {}

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

impl DbExecutor {
    pub fn new(db_url: &str) -> DbExecutor {
        DbExecutor {
            conn: PgConnection::establish(db_url).expect("Error Connecting Db"),
        }
    }
}

pub struct DocId;
//pub struct DocId{}
#[test]
fn test(){
//    let ss = DocId;
//    dbg!(ss)

}
impl Message for DocId{
    type Result = io::Result<i64>;
}

impl Handler<DocId> for DbExecutor{
    type Result = io::Result<i64>;

    fn handle(&mut self, msg: DocId, ctx: &mut Self::Context) -> Self::Result {
        use crate::schema::doc;
//        unimplemented!()
        match doc::table.load::<(i64,)>(&self.conn){
            Ok(mut items) => Ok(items.pop().unwrap().0),
            Err(_) => Err(io::Error::new(io::ErrorKind::Other, "Database error")),
        }
    }
}
