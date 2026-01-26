# AISP Open Core Workspace

This repository contains the complete AISP (AI Symbolic Protocol) implementation ecosystem organized as a unified Rust workspace.

## 📁 **Project Structure**

```
aisp-open-core/
├── 📦 aisp-rust/                           # Published crate (simple API)
├── 📦 aisp-formal-verification/           # Advanced formal verification system
│   ├── crates/aisp-core/                  # Core parsing and verification engine
│   └── crates/aisp-cli/                   # Command-line tools
├── 📦 validator/                          # Node.js/WebAssembly implementation
├── 📚 evidence/                           # Test documents and examples
├── 📝 docs/                              # Architecture Decision Records (ADRs)
└── 🔧 Cargo.toml                         # Unified workspace configuration
```

## 🚀 **Quick Start**

### Build Everything
```bash
# Build all workspace members
cargo build --workspace --release

# Run tests across all crates
cargo test --workspace

# Run the advanced CLI
./aisp-rust-pure/target/release/aisp-cli validate evidence/tic-tac-toe/spec.aisp
```

### Use Individual Components

**Simple Rust API:**
```bash
cd aisp-rust
cargo run --example basic
```

**Advanced CLI with Formal Verification:**
```bash
cd aisp-formal-verification
cargo run -- --level formal --format detailed validate ../evidence/tic-tac-toe/spec.aisp
```

**Node.js/WebAssembly:**
```bash
cd validator
npm install
npx aisp-validator validate ../evidence/tic-tac-toe/spec.aisp
```

## 🛠️ **Implementation Levels**

### 1. **aisp-rust** - Published Library
- **Purpose**: Simple, stable API for basic AISP validation
- **Published**: ✅ [crates.io](https://crates.io/crates/aisp)
- **Features**: Basic validation, quality tiers, streaming support
- **Target**: Production applications needing simple AISP validation

### 2. **aisp-formal-verification** - Advanced Engine
- **Purpose**: Complete formal verification and analysis system
- **Status**: 🔬 Research/Advanced implementation
- **Features**: 
  - Genuine mathematical proof construction
  - Ambiguity measurement with `Ambig(D) < 0.02` enforcement
  - Natural deduction theorem proving
  - Protocol state machine analysis
  - Concurrent behavior verification
- **Target**: Formal methods research, safety-critical applications

### 3. **validator** - Cross-Platform
- **Purpose**: Node.js/Browser/WebAssembly support
- **Published**: ✅ [npm](https://npmjs.com/package/aisp-validator)
- **Features**: Universal validation, browser support, WASM kernel
- **Target**: Web applications, JavaScript/TypeScript projects

## 🔧 **Workspace Configuration**

### Shared Dependencies
All crates use workspace-managed versions for consistency:
- `serde` - Serialization
- `clap` - CLI interface
- `thiserror`/`anyhow` - Error handling
- `tokio` - Async runtime
- `uuid` - Unique identifiers

### Build Profiles

| Profile | Purpose | Optimizations |
|---------|---------|---------------|
| `dev` | Development | Minimal optimization, debug info |
| `release` | Production | Full LTO, strip symbols, panic=abort |
| `cli` | Command-line tools | Release + debug symbols stripped |
| `wasm` | WebAssembly | Size optimization (`opt-level="z"`) |

### Optional Features

**Z3 Integration:**
```bash
# Enable Z3 theorem proving (requires system Z3 installation)
cargo build --features z3-verification
```

**Individual Component Features:**
- `aisp-rust`: `streaming`, `serde`, `wasm`, `z3`
- `aisp-core`: `std`, `serde`, `z3-verification`
- `aisp-cli`: `z3-verification`

## 🧪 **Testing Strategy**

### Test Organization
```bash
# Unit tests (individual crates)
cargo test -p aisp-core
cargo test -p aisp-cli

# Integration tests (workspace-level)
cargo test --workspace

# Specific test suites
cargo test test_formal_verification
cargo test test_enumeration_parsing_fix
```

### Test Categories
- **Unit Tests**: Individual component testing
- **Integration Tests**: Cross-component workflows
- **Implementation Validation**: Regression tests for improvements
- **Formal Verification Tests**: Mathematical correctness validation

## 📋 **Development Workflow**

### Building
```bash
# Fast development build
cargo build

# Optimized CLI for testing
cargo build --profile cli -p aisp-cli

# WebAssembly build
cd validator/wasm && cargo build --profile wasm --target wasm32-unknown-unknown
```

### Testing New Features
```bash
# Test parser improvements
cargo test test_enumeration_parsing_fix

# Test formal verification
./aisp-rust-pure/target/release/aisp-cli --level formal validate test.aisp

# Test ambiguity calculation
cargo test test_ambiguity_measurement
```

### Documentation
```bash
# Generate API documentation
cargo doc --workspace --open

# Check documentation
cargo doc --workspace --document-private-items
```

## 🏗️ **Architecture Overview**

### Published Components (Stable)
- **aisp-rust**: Simple validation library
- **validator**: Node.js/WebAssembly package

### Research Components (Advanced)
- **aisp-formal-verification**: Complete formal verification system
  - **aisp-core**: Advanced parsing and formal verification engine
  - **aisp-cli**: Research tools for formal analysis

### Shared Resources
- **evidence/**: Test documents and examples
- **docs/**: Architecture decisions and specifications
- **reference.md**: AISP 5.1 specification

## 🔬 **Recent Improvements**

- ✅ **Genuine Formal Verification**: Replaced placeholder logic with actual mathematical proof construction
- ✅ **Parser Enhancements**: Fixed enumeration syntax to support space-separated variants
- ✅ **Ambiguity Measurement**: Implemented measurable calculation with AISP invariant enforcement
- ✅ **Workspace Unification**: Consolidated build configuration and dependency management
- ✅ **Clean Architecture**: Removed build artifacts and organized structure

## 📚 **Documentation**

- **[AI_GUIDE.md](AI_GUIDE.md)**: Complete AISP 5.1 specification for AI agents
- **[HUMAN_GUIDE.md](HUMAN_GUIDE.md)**: Human-readable introduction to AISP
- **[docs/adr/](docs/adr/)**: Architecture Decision Records
- **[ARCHITECTURE_ANALYSIS.md](ARCHITECTURE_ANALYSIS.md)**: Technical analysis

## 🤝 **Contributing**

1. **Choose Component**: Pick aisp-rust (stable) or aisp-formal-verification (research)
2. **Follow Patterns**: Use existing code style and architecture patterns
3. **Test Changes**: Run relevant test suites
4. **Update Documentation**: Keep ADRs and guides current

## 📄 **License**

Dual licensed under MIT OR Apache-2.0.