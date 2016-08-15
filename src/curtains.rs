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
        mock_hw::move_blinds(inner.open);
        inner.open
    }
}

/// Mock out hardware interface until this can be implemented.
/// All mock_hw methods block on hardware.
mod mock_hw {
    pub fn move_blinds(open: bool) {
        if open {
            open_blinds();
        } else {
            close_blinds();
        }
    }

    pub fn open_blinds() {
        println!("Open Curtains");
    }

    pub fn close_blinds() {
        println!("Close Curtains");
    }
}