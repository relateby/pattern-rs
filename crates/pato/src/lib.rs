pub mod cli;
pub mod commands;
pub mod diagnostic_gram;
pub mod diagnostics;
pub mod editor;
pub mod extensions;
pub mod output;
pub mod schema;
pub mod skill_install;
pub mod source_map;
pub mod topic_catalog;

include!(concat!(env!("OUT_DIR"), "/skill_bundle.rs"));
