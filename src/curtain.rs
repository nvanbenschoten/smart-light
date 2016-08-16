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

    /// Moves the blinds to the specified position.
    pub fn move_blinds(&self, open: bool) {
        if open {
            self.open_blinds();
        } else {
            self.close_blinds();
        }
    }

    /// Opens the blinds if they are closed.
    #[allow(dead_code)]
    pub fn open_blinds(&self) {
        let mut inner = self.inner.write().unwrap();
        if !inner.open {
            mock_hw::open_blinds();
            inner.open = true;
        }
    }

    /// Closes the blinds if they are open.
    #[allow(dead_code)]
    pub fn close_blinds(&self) {
        let mut inner = self.inner.write().unwrap();
        if inner.open {
            mock_hw::close_blinds();
            inner.open = false;
        }
    }

    /// Toggles the blinds state.
    pub fn toggle_blinds(&self) -> bool {
        let mut inner = self.inner.write().unwrap();
        let new_state = !inner.open;
        if new_state {
            mock_hw::open_blinds();
        } else {
            mock_hw::close_blinds();
        }
        inner.open = new_state;
        new_state
    }
}

/// Mock out hardware interface until this can be implemented.
/// All mock_hw methods block on hardware.
mod mock_hw {
    pub fn open_blinds() {
        println!("Open Curtains");
    }

    pub fn close_blinds() {
        println!("Close Curtains");
    }
}
