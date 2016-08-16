use chrono::{NaiveDateTime, NaiveDate, NaiveTime, DateTime, Datelike, Local, Weekday};
use num::traits::FromPrimitive;
use postgres::{Connection, SslMode};
use postgres::error::{Error, ConnectError};
use postgres::rows::Row;

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

    pub fn new_action(&self, weekday: Weekday, time: NaiveTime, open: bool) -> Result<Action, ServiceError> {
        let day_as_int = weekday.num_days_from_monday();
        let datetime: NaiveDateTime = datetime_for_time(time);
        let rows = try!(self.connection.query("INSERT INTO actions
                                                   (id, day, time, open)
                                               VALUES
                                                   (DEFAULT, $1, $2, $3)
                                               RETURNING
                                                   id, day, time, open",
                                                &[&day_as_int, &datetime, &open])
                                       .map_err(|e| ServiceError::Exec(e)));
        assert_eq!(rows.len(), 1);
        Ok(row_to_action(rows.get(0)))
    }

    pub fn get_actions(&self) -> Result<Vec<Action>, ServiceError> {
        let rows = try!(self.connection.query("SELECT id, day, time, open FROM actions", &[])
                                       .map_err(|e| ServiceError::Exec(e)));
        let mut actions = Vec::new();
        for row in &rows {
            actions.push(row_to_action(row));
        }
        Ok(actions)
    }
}

fn row_to_action(row: Row) -> Action {
    Action {
        id:      row.get(0),
        weekday: Weekday::from_i64(row.get(1)).unwrap(),
        time:    row.get::<_, NaiveDateTime>(2).time(),
        open:    row.get(3),
    }
}

fn datetime_for_time(time: NaiveTime) -> NaiveDateTime {
    let zero_date = NaiveDate::from_ymd(2000, 1, 1);
    NaiveDateTime::new(zero_date, time)
}