use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard};
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
    alarms: HashMap<i64, AlarmAction>,
    db_srv: db::Service,
}

/// Holds an Action that is scheduled to be run in the future.
/// Dropping this struct will cancel the schedule.
struct AlarmAction {
    #[allow(dead_code)]
    action: db::Action,
    _guard: timer::Guard,
}

impl Service {
    pub fn start(curtain_mgr: &curtain::Manager) -> Result<Service, db::ServiceError> {
        let db_srv = try!(db::Service::new());
        let actions = try!(db_srv.get_actions());
        let service = Service {
            inner: Arc::new(Mutex::new(InnerService{
                timer:  timer::Timer::new(),
                alarms: HashMap::new(),
                db_srv: db_srv,
            })),
            curtain_mgr: curtain_mgr.clone(),
        };
        for action in actions {
            service.add_action(action);
        }
        Ok(service)
    }

    #[allow(dead_code)]
    pub fn new_action(&self, weekday: Weekday, time: NaiveTime, open: bool) -> Result<i64, db::ServiceError> {
        let mut inner = self.inner.lock().unwrap();
        let action = try!(inner.db_srv.new_action(weekday, time, open));
        let action_id = action.id;
        self.add_action_inner(&mut inner, action);
        Ok(action_id)
    }

    fn add_action(&self, action: db::Action) {
        let mut inner = self.inner.lock().unwrap();
        self.add_action_inner(&mut inner, action);
    }

    fn add_action_inner(&self, inner: &mut MutexGuard<InnerService>, action: db::Action) {
        let srv_clone = self.clone();
        let action_clone = action.clone();
        let guard = inner.timer.schedule(action.next_occurence(), Some(Duration::weeks(1)), move || {
            srv_clone.curtain_mgr.move_blinds(action_clone.open);
        });
        inner.alarms.insert(action.id, AlarmAction {
            action: action,
            _guard: guard,
        });
    }

    /// Drop action removes the registered action from executing.
    #[allow(dead_code)]
    pub fn drop_action(&self, action_id: i64) -> Result<bool, db::ServiceError> {
        let mut inner = self.inner.lock().unwrap();
        let deleted = try!(inner.db_srv.delete_action(action_id));
        if deleted {
            // Dropping the AlarmAction from the alarms map will cause the
            // timer::Guard to be dropped, cancelling the schedule.
            inner.alarms.remove(&action_id).unwrap();
        }
        Ok(deleted)
    }
}