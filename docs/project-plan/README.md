# JustCall Project Documentation

## Project Overview
JustCall is a zero-friction video calling app. Press a global hotkey → instantly in a call with your configured partner/group. No accounts, no sign-in, just instant connection.

## Documentation Structure

### Planning Documents
1. **[00-original-conversation.md](00-original-conversation.md)** - Original project discussion and requirements
2. **[01-setup-guide.md](01-setup-guide.md)** - Development environment setup instructions
3. **[02-detailed-project-plan.md](02-detailed-project-plan.md)** - Comprehensive implementation plan with 9 phases

## Quick Start
1. Follow the [Setup Guide](01-setup-guide.md) to install all prerequisites
2. Read through the [Project Plan](02-detailed-project-plan.md) to understand the architecture
3. Start with Phase 1, Step 1.1 (Code Generation Module)

## Key Technical Decisions
- **Framework**: Tauri (Rust backend + Web frontend)
- **Video**: Jitsi Meet (free, no accounts needed)  
- **Platforms**: macOS, Windows, Linux
- **Architecture**: Minimal modules, single state machine, no unnecessary dependencies

## Development Principles
1. The best code is the one that doesn't have to be written
2. Every function must document what uses it
3. Keep it simple - no clever abstractions
4. Test everything - unit, integration, and manual

## Repository Structure
```
justcall/
├── docs/
│   └── project-plan/     # This documentation
├── src/                  # Rust source code
│   ├── core/            # Pure utilities
│   ├── models/          # Data structures
│   ├── storage/         # Settings persistence
│   └── controllers/     # Business logic
├── src-tauri/           # Tauri-specific code
├── src/                 # Web frontend
└── tests/               # Test fixtures
```
