//! Application state management
//!
//! This module handles the global application state, including system monitoring
//! and process tracking capabilities.

use crate::monitoring::{ProcessMonitor, SystemMonitor};
use std::sync::Mutex;
use sysinfo::{System, SystemExt};

/// Global application state
///
/// Maintains thread-safe access to system information and monitoring components
#[derive(Debug)]
pub struct AppState {
    /// System information handler
    pub sys: Mutex<System>,
    /// Process monitoring component
    pub process_monitor: Mutex<ProcessMonitor>,
    /// System statistics monitoring component
    pub system_monitor: Mutex<SystemMonitor>,
}

impl AppState {
    /// Creates a new instance of the application state
    ///
    /// Initializes system monitoring and process tracking components
    ///
    /// # Returns
    ///
    /// A new `AppState` instance with initialized monitors
    pub fn new() -> Self {
        let mut sys = System::new();
        sys.refresh_all();

        Self {
            process_monitor: Mutex::new(ProcessMonitor::new()),
            system_monitor: Mutex::new(SystemMonitor::new(&sys)),
            sys: Mutex::new(sys),
        }
    }
}
