/// Storage module for persistent settings management
/// What: Handles saving/loading settings to disk
/// Why: Users need their settings to persist between app restarts
/// Used by: Main app initialization, settings UI, target management
/// Change notes: If changing file format, implement migration

pub mod settings_store;

// Re-export for convenience
pub use settings_store::SettingsStore;
