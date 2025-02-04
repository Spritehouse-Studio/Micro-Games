//! Test game to showcase the capabilities of the Spritehouse Engine.

// #![windows_subsystem = "windows"]
#![deny(
    bad_style,
    dead_code,
    improper_ctypes,
    missing_debug_implementations,
    missing_docs,
    non_shorthand_field_patterns,
    no_mangle_generic_items,
    overflowing_literals,
    path_statements,
    patterns_in_fns_without_body,
    trivial_casts,
    trivial_numeric_casts,
    unconditional_recursion,
    unused,
    unused_allocation,
    unused_comparisons,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results,
    unused_parens,
    while_true
)]

use bevy::app::App;
use spritehouse_engine::engine::EnginePlugin;

fn main() {
    _ = App::new()
        .add_plugins(EnginePlugin {
            game_name: "Micro Games".to_string(),
        })
        .run();
}
