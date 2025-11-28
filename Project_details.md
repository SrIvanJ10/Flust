# Flust - Flow-Based Programming Language Compiler

A visual flow-based programming tool that compiles to native Rust code, combining the ease of visual programming with the efficiency of compiled binaries.

## Vision

Most visual programming environments (Node-RED, Simulink, LabVIEW) rely on runtime interpretation, creating unnecessary overhead. *Flust* eliminates this by treating visual flows as a high-level language that compiles directly to native code.

Write your logic visually, execute it as a compiled binary. No interpreter, no runtime overhead, no dependencies required at execution time.

## Core Concept

```
Visual Flow Editor → Intermediate Representation → Rust Code Generation → Native Binary
```

### What makes Flust different

- **Compiled output**: Generates standalone Rust code that compiles to a single binary
- **Zero runtime cost**: No orchestration layer, no VM, pure compiled code
- **Type-safe**: Rust's type system ensures correctness at compile time
- **Linux-first**: Optimized for Linux environments
- **Minimal dependencies**: Simple flows have minimal generated code footprint
- **Open Source**: GPLv3 - fully transparent and community-driven

## Features (Planned)

### Phase 1: Core Runtime & CLI

- [ ] Flow definition format (TOML/YAML or custom DSL)
- [ ] Basic node types (input, output, arithmetic, conditional, loop)
- [ ] Data flow graph validation
- [ ] Code generation to Rust
- [ ] CLI compiler: `flust compile flow.yaml -o output`
- [ ] Type inference between connected nodes

### Phase 2: Editor & IDE Integration

- [ ] Web-based visual editor (React Flow frontend)
- [ ] Drag-and-drop node creation
- [ ] Real-time validation and error highlighting
- [ ] Live preview of generated code
- [ ] Hot reload during development

### Phase 3: Advanced Features

- [ ] Custom node definitions
- [ ] Module system and code reuse
- [ ] Async/concurrent flows
- [ ] Standard library of common operations
- [ ] Debugging integration with generated code

## Architecture

**Design principle**: Single coherent toolchain. Flust generates Rust, Flust backend is Rust. Zero impedance mismatch.

### Backend (Rust)

- **flust-core**: Parser, AST, type system, intermediate representation
- **flust-codegen**: Converts IR to idiomatic Rust code (pluggable for future language targets)
- **flust-cli**: Command-line compiler interface
- **flust-server**: REST API for the editor (Tokio-based for efficient I/O)
- **flust-validator**: Type checking and connectivity validation at multiple stages

**Performance considerations built-in:**
- Async/await via Tokio inimal thread overhead)
- No garbage collection (predictable performance)
- Zero-copy where possible
- Optimized for laptop/embedded with low battery impact

### Frontend (React Flow)

- Visual editor for flow construction
- Node library and palette
- Live code preview
- Real-time error highlighting and validation
- Project management and persistence

### Extensibility

The codebase uses trait-based design to allow future code generators:

```rust
pub trait CodeGenerator {
    fn generate(&self, ir: &IntermediateRepr) -> Result<String>;
}
```

This enables adding C++, Go, or Python backends without restructuring the core architecture.

## Target Users

- **IoT/Embedded developers**: Need rapid prototyping with compiled output
- **Systems engineers**: Want visual logic without runtime overhead
- **Researchers**: Require quick experimentation with performance
- **Educators**: Teaching programming concepts visually to beginners

## Installation & Usage

```bash
# Clone and build
git clone git@github.com:SrIvanJ10/Flust.git
cd Flust
cargo build --release

# Compile a flow
./target/release/flust compile example.yaml -o my_program

# Execute
./my_program
```

## Example Flow

A simple temperature sensor workflow:

```yaml
nodes:
  - id: sensor_read
    type: input
    label: "Temperature Input"
    
  - id: threshold_check
    type: greater_than
    value: 30
    
  - id: alert
    type: output
    label: "Temperature Alert"

connections:
  - from: sensor_read
    to: threshold_check
  - from: threshold_check
    to: alert
```

Generates optimized Rust code that you can compile to a binary.

## Why Rust?

- Type safety prevents entire classes of bugs
- Performance matches C++ without manual memory management
- Excellent ecosystem (tokio, serde, etc.)
- Cargo makes dependency management straightforward
- Generated code is readable and debuggable

## Development Status

Currently in early development. Phase 1 focuses on getting the core compilation pipeline working before building the visual editor.

## License

GPLv3 - This project is completely open source. Any derivative work must also be open source and attribute the original project.

## Contributing

Contributions welcome. Focus areas:
- Core code generation
- Type system improvements
- CLI tools
- Documentation

---

**Philosophy**: Coherence over complexity. Every feature must earn its place by providing genuine value without introducing unnecessary bloat.