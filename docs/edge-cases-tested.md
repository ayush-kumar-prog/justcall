# Edge Cases Tested

## Phase 1.1: Code Generation (crypto.rs)
- ✅ Uniqueness across 10,000 codes
- ✅ Concurrent generation from 10 threads
- ✅ Rapid sequential generation (no timing issues)
- ✅ Memory stability with 10,000 generations
- ✅ Randomness quality (no excessive patterns)
- ✅ Performance under 100ms for 1000 codes

## Phase 1.2: Room Derivation (room.rs)
- ✅ Empty string input
- ✅ Very long input (1000 chars)
- ✅ Special characters (!@#$, unicode 🦀, SQL injection attempts)
- ✅ Case sensitivity (TEST vs test vs TeSt)
- ✅ Whitespace variations (spaces, tabs, leading/trailing)
- ✅ Concurrent derivation (thread safety)
- ✅ Similar inputs produce different outputs
- ✅ Performance under 1ms per derivation

## Phase 1.3: Platform Detection (platform.rs)
- ✅ Consistent values across multiple calls
- ✅ Correct modifier key ordering (Cmd before Opt, Ctrl before Alt)
- ✅ No conflicts with common shortcuts (Ctrl+C, Cmd+S, etc)
- ✅ Unique keybinds (join ≠ hangup ≠ targets)
- ✅ Target prefix formatting (ends with + for appending numbers)
- ✅ Concurrent access thread safety
- ✅ Platform-specific behavior (Cmd on macOS, Ctrl elsewhere)
- ✅ Capability detection (permissions, features)

## Phase 2.1: Settings Schema (settings.rs)
- ✅ Default values and initialization
- ✅ JSON serialization/deserialization
- ✅ Backwards compatibility (missing fields)
- ✅ Empty strings in all fields
- ✅ Unicode characters in labels and notes
- ✅ Large settings (100 targets)
- ✅ Duplicate target IDs (allowed, validation elsewhere)
- ✅ Malformed JSON error handling
- ✅ Optional fields (notes, custom hotkeys)
- ✅ Settings equality comparison

## Phase 2.2: Settings Store (settings_store.rs)
- ✅ Missing file creates defaults
- ✅ Save and reload preserves data
- ✅ First target becomes primary automatically
- ✅ Removing primary reassigns to next target
- ✅ Update existing target
- ✅ Empty file path handling
- ✅ Corrupt JSON error recovery
- ✅ Invalid directory paths
- ✅ Concurrent file access (10 threads)
- ✅ Unicode in file paths (Chinese/emoji)
- ✅ Very large settings (1000 targets with 1KB notes each)
- ✅ Atomic save (temp file cleanup)
- ✅ Backwards compatibility (missing fields use defaults)

## Test Commands
```bash
# Run all tests
cargo test

# Run with output on failure
cargo test -- --nocapture

# Run specific module
cargo test core::crypto
cargo test core::room
cargo test core::platform
cargo test models::settings
cargo test storage::settings_store
```
