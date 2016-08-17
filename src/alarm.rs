use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use chrono::{Duration, Weekday, NaiveTime};
use timer;

use db;
use curtain;

#[derive(Clone)]
pub struct Service {
    inner: Arc<Mutex<InnerService>>,
    curtain_mgr: curtain::Manager,
}

struct InnerService {
    timer: timer::Timer,
    alarms: HashMap<i64, ScheduledAction>,
    db_srv: db::Service,
}

/// Holds an Action that is scheduled to be run in the future.
/// Dropping this struct will cancel the schedule.
struct ScheduledAction {
    #[allow(dead_code)]
    action: db::Action,
    _guard: timer::Guard,
}

impl Service {
    pub fn start(curtain_mgr: &curtain::Manager) -> Result<Service, db::ServiceError> {
        let db_srv = try!(db::Service::new());
        let service = Service {
            inner: Arc::new(Mutex::new(InnerService{
                timer:  timer::Timer::new(),
                alarms: HashMap::new(),
                db_srv: db_srv,
            })),
            curtain_mgr: curtain_mgr.clone(),
        };
        {
            let mut inner = service.inner.lock().unwrap();
            try!(inner.load_initial_actions(&service));
        }
        Ok(service)
    }

    #[allow(dead_code)]
    pub fn new_action(&self, weekday: Weekday, time: NaiveTime, open: bool) -> Result<i64, db::ServiceError> {
        let mut inner = self.inner.lock().unwrap();
        inner.create_action(self, weekday, time, open)
    }

    /// Drop action removes the registered action from executing.
    #[allow(dead_code)]
    pub fn drop_action(&self, action_id: i64) -> Result<bool, db::ServiceError> {
        let mut inner = self.inner.lock().unwrap();
        inner.drop_action(action_id)
    }
}

impl InnerService {
    fn load_initial_actions(&mut self, srv: &Service) -> Result<(), db::ServiceError> {
        let actions = try!(self.db_srv.get_actions());
        for action in actions {
            self.add_action(srv, action);
        }
        Ok(())
    }

    fn create_action(&mut self, srv: &Service, weekday: Weekday, time: NaiveTime, open: bool) -> Result<i64, db::ServiceError> {
        let action = try!(self.db_srv.new_action(weekday, time, open));
        let action_id = action.id;
        self.add_action(srv, action);
        Ok(action_id)
    }

    fn add_action(&mut self, srv: &Service, action: db::Action) {
        let srv_clone = srv.clone();
        let action_clone = action.clone();
        let guard = self.timer.schedule(action.next_occurence(), Some(Duration::weeks(1)), move || {
            srv_clone.curtain_mgr.move_blinds(action_clone.open);
        });
        self.alarms.insert(action.id, ScheduledAction {
            action: action,
            _guard: guard,
        });
    }

    fn drop_action(&mut self, action_id: i64) -> Result<bool, db::ServiceError> {
        let deleted = try!(self.db_srv.delete_action(action_id));
        if deleted {
            // Dropping the ScheduledAction from the alarms map will cause the
            // timer::Guard to be dropped, cancelling the schedule.
            self.alarms.remove(&action_id).unwrap();
        }
        Ok(deleted)
    }
}