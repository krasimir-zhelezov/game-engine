# Skalora Game Engine
Welcome to the Skalora 2D Game Engine! Skalora is built on a robust Entity-Component-System (ECS) architecture, written in Rust, and uses ```winit``` and ```wgpu``` under the hood. This guide will walk you through the core concepts and show you how to build your first game.

[View Todo List](TODO.md)

## How to run it

### Requirements
* rustc = 1.90.0
* cargo = 1.90.0

### Commands
1. Clone the repository
```bash
git clone https://github.com/krasimir-zhelezov/game-engine.git
cd game-engine
```

2. View available examples
```bash
ls examples
```

3. Run an example
```bash
cargo run --example <name_of_example>
```

## Documentation
Use this command to generate the documentation.

```bash
cargo doc --no-deps --open
```