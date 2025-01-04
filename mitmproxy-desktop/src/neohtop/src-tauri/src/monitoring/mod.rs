//! System monitoring functionality
//!
//! This module provides types and functionality for monitoring system resources
//! and processes. It includes process monitoring, system statistics collection,
//! and data structures for representing system state.

mod process_monitor;
mod system_monitor;
mod types;

pub use process_monitor::ProcessMonitor;
pub use system_monitor::SystemMonitor;
pub use types::*; // Re-export all types
