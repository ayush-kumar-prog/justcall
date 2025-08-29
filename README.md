# JustCall

Zero-friction video calling. Press a hotkey → instantly in a call.

## Quick Start

```bash
# Run the app
cargo run

# Run all tests
cargo test

# Run specific module tests
cargo test core::crypto      # Phase 1.1: Code generation ✅
cargo test core::room        # Phase 1.2: Room derivation ✅
cargo test core::platform    # Phase 1.3: Platform detection ✅
cargo test models::settings  # Phase 2.1: Settings schema ✅
```

## Project Structure

```
src/
├── core/           # Pure utilities (no dependencies)
│   ├── crypto.rs   # Code generation ✅
│   ├── room.rs     # Room ID derivation ✅
│   └── platform.rs # OS-specific defaults ✅
├── models/         # Data structures
│   └── settings.rs # Settings schema ✅
├── storage/        # Settings persistence
└── main.rs         # Entry point
```

## Development

See [docs/project-plan/](docs/project-plan/) for detailed implementation plan.
