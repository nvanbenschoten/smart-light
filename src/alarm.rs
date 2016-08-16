use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use chrono::Duration;
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
    alarms: HashMap<i32, AlarmAction>,
    db_srv: db::Service,
}

/// Holds an Action that is scheduled to be run in the future.
/// Dropping this struct will cancel the schedule.
struct AlarmAction {
    action: db::Action,
    _expire: timer::Guard,
}

impl Service {
    pub fn start(curtain_mgr: &curtain::Manager) -> Result<Service, db::ServiceError> {
        let db_srv = try!(db::Service::new());
        let mut service = Service {
            inner: Arc::new(Mutex::new(InnerService{
                timer:  timer::Timer::new(),
                alarms: HashMap::new(),
                db_srv: db_srv,
            })),
            curtain_mgr: curtain_mgr.clone(),
        };
        Ok(service)
    }

    fn add_action(&mut self, action: db::Action) {
        let srv_clone = self.clone();
        let action_clone = action.clone();
        let mut inner = self.inner.lock().unwrap();
        let guard = inner.timer.schedule(action.next_occurence(), Some(Duration::weeks(1)), move || {
            srv_clone.curtain_mgr.move_blinds(action_clone.open);
        });
        inner.alarms.insert(action.id, AlarmAction {
            action: action,
            _expire: guard,
        });
    }

    /// Drop action removes the registered action from executing.
    fn drop_action(&mut self, action_id: i32) -> Option<db::Action> {
        let mut inner = self.inner.lock().unwrap();
        inner.alarms.remove(&action_id).map(|a| a.action)
    }
}