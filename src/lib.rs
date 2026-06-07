//! # line-sweep-rs
//!
//! Computational geometry algorithms based on the line sweep paradigm.
//!
//! # Modules
//!
//! - [`segment`] — Line segment intersection detection
//! - [`closest_pair`] — Closest pair of points (divide-and-conquer)
//! - [`rectangle`] — Axis-aligned rectangle intersection
//! - [`event`] — Sweep event types
//! - [`status`] — Status structure for sweep line

pub mod event;
pub mod status;
pub mod segment;
pub mod closest_pair;
pub mod rectangle;
