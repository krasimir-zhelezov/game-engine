# Skalora Game Engine
Welcome to the Skalora 2D Game Engine! Skalora is built on a robust Entity-Component-System (ECS) architecture, written in Rust, and uses ```winit``` and ```wgpu``` under the hood. This guide will walk you through the core concepts and show you how to build your first game.

[View Todo List](TODO.md)

## How to run it

### Requirements
* rustc = 1.90.0
* cargo = 1.90.0

### Commands
```bash
git clone https://github.com/krasimir-zhelezov/game-engine.git
cd game-engine
cargo run
```

## Getting Started Guide

### Core Architecture (ECS)
Skalora separates data from logic to ensure high performance and modularity. You will interact with four main pillars:

* Entities: Unique IDs representing objects in your game world (e.g., the player, an enemy, a camera). They possess no logic or data on their own.

* Components: Pure data structures attached to Entities (e.g., ```Transform```, ```Velocity```, ```Renderable```).

* Systems: The logic that runs every frame. Systems query for Entities that have specific Components and update them.

* Resources: Global, singleton data accessible across the engine (e.g., ```InputState```, ```CollisionEvents```, ```CameraState```).

### 1. Initializing the World
Everything in Skalora lives inside the ```World```. When the ```World``` is instantiated, it automatically registers the built-in components and systems.

To set up your game scene, you interact primarily with the ```World```'s ```entity_manager``` and ```components_store```.

#### Built-in Components
* Transform: Handles position, scale, and rotation.

* Renderable: Handles textures, colors, and shapes.

* Camera: Defines the viewport, zoom, and projection.

* Collider: Defines collision shapes (e.g., Box).

* Velocity: Handles movement vectors.

* Tag: Allows for easy identification of specific entities.

### 2. Creating Entities and Adding Components
To add objects to your game, you first create an entity ID, then attach the relevant components to it. Here is how you spawn a player character:

```rust
// Create a new entity id
let player_id = world.entity_manager.create_entity();

// Add spatial data (Transform)
world.components.add_component(player_id, Transform {
    position: Position { x: 0.0, y: 0.0 },
    scale: Scale { x: 2.0, y: 2.0 },
    rotation: 0.0,
});

// Add visual data (Renderable)
world.components.add_component(
    player_id, 
    Renderable::new_texture(asset_manager.get_texture("player.png").unwrap())
);

// Add engine-specific behavior (Collider)
world.components.add_component(player_id, Collider {
    shape: ColliderShape::Box { width: 1.0, height: 1.0 },
});
```

### 3. Writing Custom Systems
Systems contain your game logic. They run every frame and mutate component data. To create a custom system, you must implement the System trait.

#### Example: Player Movement Logic
When querying components, you must manage Rust's borrowing rules carefully. The standard pattern in Skalora is to do a two-pass approach:

1. Read immutable data to figure out what needs to change.

2. Mutate the target data based on your findings.

```rust
use winit::keyboard::KeyCode;
use crate::{
    systems::{system::System, input_system::InputState},
    components::{custom::player_controller::PlayerController, transform::Transform},
};

pub struct PlayerMovementSystem;

impl PlayerMovementSystem {
    pub fn new() -> Self { Self }
}

impl System for PlayerMovementSystem {
    fn update(&mut self, world: &mut crate::world::WorldView) {
        // Access Global Resources
        let input = world.resources.get::<InputState>().expect("InputState is missing");

        // Calculate intended movement vector
        let mut delta_x = 0.0;
        let mut delta_y = 0.0;

        if input.is_key_held(KeyCode::KeyW) { delta_y += 1.0; }
        if input.is_key_held(KeyCode::KeyS) { delta_y -= 1.0; }
        if input.is_key_held(KeyCode::KeyA) { delta_x -= 1.0; }
        if input.is_key_held(KeyCode::KeyD) { delta_x += 1.0; }

        if delta_x == 0.0 && delta_y == 0.0 { return; }

        let mut active_movements = Vec::new();

        // PASS 1: Immutable Scope
        // Collect entities that have a PlayerController component
        {
            let player_controllers = world.components.get_component::<PlayerController>();
            for (id, player_controller_opt) in player_controllers.iter().enumerate() {
                if let Some(player_controller) = player_controller_opt {
                    active_movements.push((id, player_controller.movement_speed));
                }
            }
        }

        // PASS 2: Mutable Scope
        // Apply the calculated movement to those entities' Transform components
        let mut transforms = world.components.get_component_mut::<Transform>();
        for (id, speed) in active_movements {
            if let Some(Some(transform)) = transforms.get_mut(id) {
                transform.position.x += delta_x * speed;
                transform.position.y += delta_y * speed;
            }
        }
    }
}
```

NOTE: Once you create a new system, don't forget to register it in your World::new setup block using world.systems.add_system(Box::new(YourCustomSystem::new()));.

### 4. Input and Events
Skalora automatically captures hardware events (keyboard, mouse motion, scrolling, clicks) via winit and pipes them into the InputSystem and global InputState resource.

To check for input anywhere in your game loop, query the InputState resource:

```rust
let input = world.resources.get::<InputState>().unwrap();

if input.is_key_held(KeyCode::Space) {
    // Perform jump logic
}
```

Collision events are similarly stored in a global resource and cleared at the end of every frame. You can query world.resources.get::<CollisionEvents>() to react to overlapping colliders.

### 5. Running the Application
To start the engine, you initialize the App and pass it to the winit event loop in your main.rs file.

```rust
use winit::{error::EventLoopError, event_loop::EventLoop};
use crate::app::App;

fn main() -> Result<(), EventLoopError> {
    let event_loop = EventLoop::with_user_event().build().unwrap();
    let mut app = App::new();

    // Begins the application lifecycle and engine loop
    event_loop.run_app(&mut app)?;

    Ok(())
}
```