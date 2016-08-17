use chrono::*;
use num::traits::FromPrimitive;
use postgres::{Connection, SslMode};
use postgres::error::{Error, ConnectError};
use postgres::rows::Row;

pub struct Service {
    connection: Connection,
}

#[derive(Clone, Debug)]
pub struct Action {
    pub id:      i64,
    pub open:    bool,

    weekday: Weekday,
    time:    NaiveTime,
}

impl Action {
    pub fn next_occurence(&self) -> DateTime<Local> {
        self.next_occurence_from_datetime(Local::now())
    }

    fn next_occurence_from_datetime<Tz: TimeZone>(&self, datetime: DateTime<Tz>) -> DateTime<Tz> {
        let mut date = datetime.date();
        if datetime.weekday() == self.weekday {
            // Day is today. Is the next occurence later today or next week?
            if datetime.time() < self.time {
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
        let day_as_int = weekday.num_days_from_monday() as i64;
        let datetime = datetime_for_time(time);
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

    pub fn delete_action(&self, id: i64) -> Result<bool, ServiceError> {
        self.connection.execute("DELETE FROM actions WHERE id = $1", &[&id])
                       .map(|count| count > 0)
                       .map_err(|e| ServiceError::Exec(e))
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

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::*;

    #[test]
    fn test_next_occurence_from_datetime() {
        struct NextOccurenceTest {
            alarm_weekday: Weekday,
            alarm_time:    NaiveTime,
            cur_time:      DateTime<UTC>,
            exp_time:      DateTime<UTC>,
        }
        let tests = vec![
            // Later on in the same week.
            NextOccurenceTest {
                alarm_weekday: Weekday::Wed,
                alarm_time:    NaiveTime::from_hms(19, 30, 9),
                cur_time:      DateTime::from_utc(
                    NaiveDateTime::new(
                        NaiveDate::from_ymd(2016, 8, 15) /* Mon */,
                        NaiveTime::from_hms(12, 0, 0)
                    ),
                    UTC,
                ),
                exp_time:      DateTime::from_utc(
                    NaiveDateTime::new(
                        NaiveDate::from_ymd(2016, 8, 17) /* Wed */,
                        NaiveTime::from_hms(19, 30, 9)
                    ),
                    UTC,
                ),
            },
            // In the following week.
            NextOccurenceTest {
                alarm_weekday: Weekday::Mon,
                alarm_time:    NaiveTime::from_hms(9, 30, 9),
                cur_time:      DateTime::from_utc(
                    NaiveDateTime::new(
                        NaiveDate::from_ymd(2016, 8, 17) /* Wed */,
                        NaiveTime::from_hms(12, 0, 0)
                    ),
                    UTC,
                ),
                exp_time:      DateTime::from_utc(
                    NaiveDateTime::new(
                        NaiveDate::from_ymd(2016, 8, 22) /* Mon */,
                        NaiveTime::from_hms(9, 30, 9)
                    ),
                    UTC,
                ),
            },
            // Same day and later. Should be the same day.
            NextOccurenceTest {
                alarm_weekday: Weekday::Wed,
                alarm_time:    NaiveTime::from_hms(19, 30, 9),
                cur_time:      DateTime::from_utc(
                    NaiveDateTime::new(
                        NaiveDate::from_ymd(2016, 8, 17) /* Wed */,
                        NaiveTime::from_hms(12, 0, 0)
                    ),
                    UTC,
                ),
                exp_time:      DateTime::from_utc(
                    NaiveDateTime::new(
                        NaiveDate::from_ymd(2016, 8, 17) /* Wed */,
                        NaiveTime::from_hms(19, 30, 9)
                    ),
                    UTC,
                ),
            },
            // Same day but earlier. Should go almost an entire week.
            NextOccurenceTest {
                alarm_weekday: Weekday::Wed,
                alarm_time:    NaiveTime::from_hms(9, 30, 9),
                cur_time:      DateTime::from_utc(
                    NaiveDateTime::new(
                        NaiveDate::from_ymd(2016, 8, 17) /* Wed */,
                        NaiveTime::from_hms(12, 0, 0)
                    ),
                    UTC,
                ),
                exp_time:      DateTime::from_utc(
                    NaiveDateTime::new(
                        NaiveDate::from_ymd(2016, 8, 24) /* Wed */,
                        NaiveTime::from_hms(9, 30, 9)
                    ),
                    UTC,
                ),
            },
            // Same day, same time. Should go a week.
            NextOccurenceTest {
                alarm_weekday: Weekday::Wed,
                alarm_time:    NaiveTime::from_hms(12, 0, 0),
                cur_time:      DateTime::from_utc(
                    NaiveDateTime::new(
                        NaiveDate::from_ymd(2016, 8, 17) /* Wed */,
                        NaiveTime::from_hms(12, 0, 0)
                    ),
                    UTC,
                ),
                exp_time:      DateTime::from_utc(
                    NaiveDateTime::new(
                        NaiveDate::from_ymd(2016, 8, 24) /* Wed */,
                        NaiveTime::from_hms(12, 0, 0)
                    ),
                    UTC,
                ),
            },
        ];

        for t in tests {
            let action = Action {
                id:      0,
                open:    false,
                weekday: t.alarm_weekday,
                time:    t.alarm_time,
            };
            assert_eq!(t.exp_time, action.next_occurence_from_datetime(t.cur_time));
        }
    }
}