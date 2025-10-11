use std::{any::{Any, TypeId}, collections::HashMap, time::{Duration, Instant}};

use wgpu::naga::Type;
use winit::{event::{self, ElementState, KeyEvent}, keyboard::{self, Key, KeyCode, PhysicalKey}};

use crate::{components::{component_store::ComponentStore, transform::{Position, Scale, Transform}}, entities::entity::Entity, resources::resource_store::ResourceStore, systems::{input_system::{self, InputState, InputSystem}, system::System, system_manager::SystemManager}};

pub struct World {
    pub running: bool,
    pub last_update: Instant,
    pub accumulator: Duration,
    pub resources: ResourceStore,
    pub systems: SystemManager,
    pub next_id: u32,
    pub components: ComponentStore, // Component Type to (Entity ID to Component)
    pub player: Option<Entity>,
}

pub struct WorldView<'a> {
    pub resources: &'a mut ResourceStore,
}

impl World {
    pub fn new() -> Self {
        let mut world = Self {
            running: true,
            last_update: Instant::now(),
            accumulator: Duration::ZERO,
            resources: ResourceStore::new(),
            systems: SystemManager::new(),
            next_id: 0,
            components: ComponentStore::new(),
            player: None,
        };

        world.resources.insert(InputState::new());
        world.systems.add_system(Box::new(InputSystem::new()));

        world.player = Some(world.create_entity());

        if let Some(player) = world.player {
            world.components.add_component(player, Transform {
                position: Position { x: 0.0, y: 0.0 },
                scale: Scale { x: 1.0, y: 1.0 },
            });
        }

        world
    }

    pub fn update(&mut self) {
        self.systems.update(&mut WorldView {
            resources: &mut self.resources,
        });

        let input = self.resources.get::<InputState>().unwrap();
        if input.is_key_pressed(KeyCode::KeyW) {
            if let Some(player) = self.player {
                self.components.get_component_mut::<Transform>(&player).map(|transform| {
                    transform.position.y += 1.0;
                });
            }
        }

        if let Some(player) = self.player {
            let transform = self.components.get_component::<Transform>(&player).unwrap();
            println!("Player Position: ({}, {})", transform.position.x, transform.position.y);
        }
    }

    pub fn render(&self) {
        // TODO: render logic
    }

    pub fn handle_keyboard_input(&mut self, event: &KeyEvent) {
        let Self { resources, systems, .. } = self;

        let input_system = systems.get_system_mut::<InputSystem>().unwrap();

        let mut view = WorldView {
            resources,
        };

        input_system.handle_keyboard_input(&mut view, event);
    }

    pub fn handle_mouse_input(&self) {
        todo!();
    }

    pub fn create_entity(&mut self) -> Entity {
        self.next_id += 1;

        Entity {
            id: self.next_id,
        }
    }
}