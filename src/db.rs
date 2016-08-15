use postgres::{Connection, SslMode};
use postgres::error::{Error, ConnectError};
use chrono::{DateTime, Local};
use curtains::Manager;

pub struct Service {
    curtain_mgr: Manager,
    connection: Connection,
}

#[allow(dead_code)]
#[repr(u32)]
#[derive(Clone, Copy, Debug)]
pub enum Weekday {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

#[derive(Debug)]
pub struct Action {
    day: Weekday,
    time: DateTime<Local>,
    open: bool,
}

#[derive(Debug)]
pub enum ServiceError {
    Connect(ConnectError),
    Exec(Error),
}

impl Service {
    pub fn new(curtain_mgr: &Manager) -> Result<Service, ServiceError> {
        let conn = try!(Connection::connect("postgresql://root@localhost:26257/smart_light", SslMode::None)
            .map_err(|e| ServiceError::Connect(e)));
        try!(conn.execute("CREATE TABLE IF NOT EXISTS actions (
            day INT,
            time TIMESTAMP,
            open BOOL,
            PRIMARY KEY (day, time)
        )", &[]).map_err(|e| ServiceError::Exec(e)));
        Ok(Service {
            curtain_mgr: curtain_mgr.clone(),
            connection: conn,
        })
    }

    // pub fn write_action() -> Result<(), Error> {

    // }
}