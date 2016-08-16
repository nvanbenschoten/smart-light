use chrono::{NaiveDateTime, NaiveDate, NaiveTime, DateTime, Datelike, Local, Weekday};
use num::traits::FromPrimitive;
use postgres::{Connection, SslMode};
use postgres::error::{Error, ConnectError};

pub struct Service {
    connection: Connection,
}

#[derive(Clone, Debug)]
pub struct Action {
    pub id:      i32,
    pub open:    bool,

    weekday: Weekday,
    time:    NaiveTime,
}

impl Action {
    pub fn next_occurence(&self) -> DateTime<Local> {
        let now = Local::now();
        let mut date = now.date();
        if now.weekday() == self.weekday {
            // Day is today. Is the next occurence later today or next week?
            if now.time() > self.time {
                return date.and_time(self.time).unwrap();
            } else {
                date = date.succ();
            }
        }
        while date.weekday() != self.weekday {
            date = date.succ();
        }
        date.and_time(self.time).unwrap()
    }
}

#[derive(Debug)]
pub enum ServiceError {
    Connect(ConnectError),
    Exec(Error),
}

impl Service {
    pub fn new() -> Result<Service, ServiceError> {
        let conn = try!(Connection::connect("postgresql://root@localhost:26257/smart_light", SslMode::None)
            .map_err(|e| ServiceError::Connect(e)));
        try!(conn.execute("CREATE TABLE IF NOT EXISTS actions (
                id   SERIAL PRIMARY KEY,
                day  INT NOT NULL,
                time TIMESTAMP NOT NULL,
                open BOOL NOT NULL,
                UNIQUE INDEX (day, time)
            )", &[])
            .map_err(|e| ServiceError::Exec(e)));
        Ok(Service {
            connection: conn,
        })
    }

    #[allow(dead_code)]
    pub fn write_action(&self, action: &Action) -> Result<(), ServiceError> {
        let day_as_int = action.weekday.num_days_from_monday();
        let datetime: NaiveDateTime = datetime_for_time(action.time);
        self.connection
            .execute("INSERT INTO actions (id, day, time, open) VALUES ($1, $2, $3)",
                     &[&action.id, &day_as_int, &datetime, &action.open])
            .map(|_| ())
            .map_err(|e| ServiceError::Exec(e))
    }

    pub fn get_actions(&self) -> Result<Vec<Action>, ServiceError> {
        let rows = try!(self.connection.query("SELECT id, day, time, open FROM actions", &[])
                                       .map_err(|e| ServiceError::Exec(e)));
        let mut actions = Vec::new();
        for row in &rows {
            actions.push(Action {
                id:      row.get(0),
                weekday: Weekday::from_i64(row.get(1)).unwrap(),
                time:    row.get::<_, NaiveDateTime>(2).time(),
                open:    row.get(3),
            });
        }
        Ok(actions)
    }
}

fn datetime_for_time(time: NaiveTime) -> NaiveDateTime {
    let zero_date = NaiveDate::from_ymd(2000, 1, 1);
    return NaiveDateTime::new(zero_date, time);
}