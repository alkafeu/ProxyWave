//! Core engines orchestration logic will live here.

pub mod error;
pub mod engines;

pub use engines::EngineAdapter;

pub fn init() {
    println!("Core initialized");
} 