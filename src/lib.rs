//! Wrapper to standard library sleep functionality. 
//! 
//! ## Why
//! I find myself always needing the sleep functionality given by the standard
//! library, but I always need to search how it works, so I made this to make my
//! life easier (_not_ that it contributes much really).
//! 
//! ## Usage
//! In `Cargo.toml`:
//! ```
//! [dependencies]
//! snor = "1.0.0"
//! ```
//! 
//! Used in a crate as:
//! ```
//! use snor;
//! 
//! // Sleep for 10 seconds.
//! snor::sleep_sec(10);
//! 
//! // Sleep for 100 milliseconds.
//! snor::sleep_ms(100);
//! 
//! // Sleep for 1000 microseconds.
//! snor::sleep_micros(1000);
//! 
//! //Sleep for 10000 nanoseconds.
//! snor::sleep_nanos(10000);
//! ```

use std::{thread, time};

/// Sleep with second granularity.
pub fn sleep_sec(sec: u64) {
    let sec = time::Duration::from_secs(sec);
    thread::sleep(sec);
}

/// Sleep with millisecond granularity.
pub fn sleep_ms(ms: u64) {
    let ms = time::Duration::from_millis(ms);
    thread::sleep(ms);
}

/// Sleep with microsecond granularity.
pub fn sleep_micros(micros: u64) {
    let micros = time::Duration::from_micros(micros);
    thread::sleep(micros);
}

/// Sleep with nanosecond granularity.
pub fn sleep_nanos(nanos: u64) {
    let nanos = time::Duration::from_nanos(nanos);
    thread::sleep(nanos);
}