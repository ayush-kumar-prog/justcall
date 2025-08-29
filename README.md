# JustCall

Zero-friction video calling. Press a hotkey → instantly in a call.

## Quick Start

```bash
# Run the app
cargo run

# Run all tests
cargo test

# Run specific module tests
cargo test core::crypto           # Phase 1.1: Code generation ✅
cargo test core::room             # Phase 1.2: Room derivation ✅
cargo test core::platform         # Phase 1.3: Platform detection ✅
cargo test models::settings       # Phase 2.1: Settings schema ✅
cargo test storage::settings_store # Phase 2.2: Settings persistence ✅
cargo test core::call_state       # Phase 2.3: Call state machine ✅
```

## Project Structure

```
src/
├── core/           # Pure utilities (no dependencies)
│   ├── crypto.rs   # Code generation ✅
│   ├── room.rs     # Room ID derivation ✅
│   ├── platform.rs # OS-specific defaults ✅
│   └── call_state.rs # Call lifecycle states ✅
├── models/         # Data structures
│   └── settings.rs # Settings schema ✅
├── storage/        # Settings persistence
│   └── settings_store.rs # Load/save settings ✅
└── main.rs         # Entry point
```

## Development

See [docs/project-plan/](docs/project-plan/) for detailed implementation plan.
