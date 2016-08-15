use std::sync::{Arc, Mutex};
use chrono::{DateTime, Local};

#[derive(Clone)]
pub struct Manager {
    inner: Arc<Mutex<InnerManager>>,
}

struct InnerManager {
    open: bool,
    last_action: Option<DateTime<Local>>,
}

impl Manager {
    pub fn new() -> Manager {
        Manager {
            inner: Arc::new(Mutex::new(InnerManager {
                open: false,
                last_action: None,
            })),
        }
    }

    pub fn is_open(&self) -> bool {
        self.inner.lock().unwrap().open
    }

    pub fn toggle(&self) -> bool {
        let mut inner = self.inner.lock().unwrap();
        inner.open = !inner.open;
        inner.open
    }
}