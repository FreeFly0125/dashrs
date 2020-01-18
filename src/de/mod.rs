//! Module containing serde deserializers for the various custom data formats RobTop uses.
//!
//! All of these deserializers have the goal to use zero-allocations for maximum efficiency

pub mod error;
pub mod indexed;
pub mod thunk;
