use std::sync::{Arc, RwLock};
use chrono::{DateTime, Local};

#[derive(Clone)]
pub struct Manager {
    inner: Arc<RwLock<InnerManager>>,
}

struct InnerManager {
    open: bool,

    #[allow(dead_code)]
    last_action: Option<DateTime<Local>>,
}

impl Manager {
    pub fn new() -> Manager {
        Manager {
            inner: Arc::new(RwLock::new(InnerManager {
                open: false,
                last_action: None,
            })),
        }
    }

    /// Returns if the blinds are open.
    pub fn is_open(&self) -> bool {
        self.inner.read().unwrap().open
    }

    /// Toggles the blinds state.
    pub fn toggle(&self) -> bool {
        let mut inner = self.inner.write().unwrap();
        let new_state = !inner.open;
        mock_hw::move_blinds(new_state);
        inner.open = new_state;
        new_state
    }

    /// Opens the blinds if they are closed.
    #[allow(dead_code)]
    pub fn open(&self) {
        let mut inner = self.inner.write().unwrap();
        if !inner.open {
            mock_hw::open_blinds();
            inner.open = true;
        }
    }

    /// Closes the blinds if they are open.
    #[allow(dead_code)]
    pub fn close(&self) {
        let mut inner = self.inner.write().unwrap();
        if inner.open {
            mock_hw::close_blinds();
            inner.open = false;
        }
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
