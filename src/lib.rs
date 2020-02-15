#![deny(
    const_err,
    illegal_floating_point_literal_pattern,
    late_bound_lifetime_arguments,
    non_camel_case_types,
    non_shorthand_field_patterns,
    non_snake_case,
    bare_trait_objects,
    missing_debug_implementations,
    missing_copy_implementations,
    unused_extern_crates,
    stable_features,
    unknown_lints,
    unused_features,
    unreachable_code,
    unreachable_patterns,
    unused_allocation,
    unused_attributes,
    unused_must_use,
    unused_mut,
    while_true,
    unused_imports,
    unconditional_recursion,
    unknown_lints,
    unused_parens,
    non_upper_case_globals,
    path_statements,
    patterns_in_fns_without_body,
    renamed_and_removed_lints,
    type_alias_bounds
)]

pub mod model;
pub(crate) mod serde;
pub mod util;

pub use crate::serde::{from_robtop_str, to_robtop_data, DeError, PercentDecoded, SerError, Thunk};
