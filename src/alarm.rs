use std::collections::HashMap;
use db;
use curtain;

pub struct Service {
    db_srv: db::Service,
    alarms: HashMap<i32, AlarmAction>,
}

struct AlarmAction {
    action: db::Action,
}

impl Service {
    pub fn start(curtain_mgr: &curtain::Manager) -> Result<Service, db::ServiceError> {
        let db_srv = try!(db::Service::new());
        Ok(Service {
            db_srv: db_srv,
            alarms: HashMap::new(),
        })
    }
}