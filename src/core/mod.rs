/// Core module containing pure utility functions with zero external dependencies
/// What: Houses crypto, room derivation, and platform utilities
/// Why: Keep core logic separate from framework-specific code (Tauri, UI, etc)
/// Used by: main.rs, future CallController, SettingsStore
/// Change notes: If adding new submodules, update parent mod.rs files

pub mod crypto;

// Re-export main functions for cleaner imports
// Usage: use justcall::core::generate_code_base32_100b;
pub use crypto::generate_code_base32_100b;
