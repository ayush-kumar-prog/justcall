/// Core module containing pure utility functions with zero external dependencies
/// What: Houses crypto, room derivation, and platform utilities
/// Why: Keep core logic separate from framework-specific code (Tauri, UI, etc)
/// Used by: main.rs, future CallController, SettingsStore
/// Change notes: If adding new submodules, update parent mod.rs files

pub mod crypto;
pub mod room;
pub mod platform;
pub mod call_state;

// Re-export main functions for cleaner imports
// Usage: use justcall::core::{generate_code_base32_100b, room_id_from_code, get_default_keybinds};
pub use crypto::generate_code_base32_100b;
pub use room::room_id_from_code;
pub use platform::{get_default_keybinds, get_platform_name, get_platform_capabilities};
pub use call_state::CallState;
