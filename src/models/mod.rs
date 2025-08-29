/// Data models for app configuration and state
/// What: Defines all serializable structures for settings persistence
/// Why: Need structured data that can be saved/loaded from disk
/// Used by: SettingsStore (Phase 2.2), UI components, CallController
/// Change notes: Adding fields is safe, removing/renaming needs migration

pub mod settings;

// Re-export main types for convenience
pub use settings::{Settings, Target, TargetType, CallDefaults};
