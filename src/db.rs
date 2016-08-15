use postgres::{Connection, SslMode};
use postgres::error::{Error, ConnectError};
use num::traits::FromPrimitive;
use chrono::{DateTime, Local, Weekday};
use curtain::Manager;

#[allow(dead_code)]
pub struct Service {
    curtain_mgr: Manager,
    connection: Connection,
}

#[allow(dead_code)]
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
            )", &[])
            .map_err(|e| ServiceError::Exec(e)));
        Ok(Service {
            curtain_mgr: curtain_mgr.clone(),
            connection: conn,
        })
    }

    #[allow(dead_code)]
    pub fn write_action(&self, action: &Action) -> Result<(), Error> {
        self.connection
            .execute("INSERT INTO actions (day, time, open) VALUES ($1, $2, $3)",
                     &[&action.day.num_days_from_monday(), &action.time, &action.open])
            .map(|_| ())
    }

    #[allow(dead_code)]
    pub fn get_actions(&self) -> Result<Vec<Action>, Error> {
        let rows = try!(self.connection.query("SELECT day, time, open FROM actions", &[]));
        let mut actions = Vec::new();
        for row in &rows {
            actions.push(Action {
                day:  Weekday::from_i64(row.get(0)).unwrap(),
                time: row.get(1),
                open: row.get(2),
            });
        }
        Ok(actions)
    }
}
