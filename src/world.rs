use std::time::{Duration, Instant};

use winit::{event::{self, ElementState, KeyEvent}, keyboard::{self, Key, KeyCode, PhysicalKey}};

use crate::{resources::resource_store::ResourceStore, systems::{input_system::{self, InputState, InputSystem}, system::System, system_manager::SystemManager}};

pub struct World {
    pub running: bool,
    pub last_update: Instant,
    pub accumulator: Duration,
    pub resources: ResourceStore,
    pub systems: SystemManager,
    // pub entities: Vec<Entity>, // TODO: Future implementation for entities
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
            // entities: Vec::new(),
        };

        world.resources.insert(InputState::new());
        world.systems.add_system(Box::new(InputSystem::new()));

        world
    }

    pub fn update(&mut self) {
        self.systems.update(&mut WorldView {
            resources: &mut self.resources,
        });

        let input = self.resources.get::<InputState>().unwrap();

        if input.is_key_pressed(KeyCode::KeyK) {
            println!("K is pressed");
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
}