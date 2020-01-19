//! Module containing all data models Geometry Dash uses
//!
//! For each data model there are two versions:
//! * A `Raw...` version which is the deserialization target. It can be constructed without any
//!   allocations at all and references the input it was deserialized from. Furthermore, these are a
//!   one-to-one mapping from response data into rust structures, meaning they also act as
//!   documentation of RobTop's data formats.
//! * A "Owned" version that owns all its fields
//!
//! The raw version can be converted into the owned version by cloning all the fields. The owned
//! version can produce a raw version by borrowing all fields (roughly speaking).

pub mod creator;
pub mod level;
pub mod song;
