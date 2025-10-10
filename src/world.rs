use std::{any::{Any, TypeId}, collections::HashMap, time::{Duration, Instant}};

use wgpu::naga::Type;
use winit::{event::{self, ElementState, KeyEvent}, keyboard::{self, Key, KeyCode, PhysicalKey}};

use crate::{components::transform::{Position, Scale, Transform}, entities::entity::Entity, resources::resource_store::ResourceStore, systems::{input_system::{self, InputState, InputSystem}, system::System, system_manager::SystemManager}};

pub struct World {
    pub running: bool,
    pub last_update: Instant,
    pub accumulator: Duration,
    pub resources: ResourceStore,
    pub systems: SystemManager,
    pub next_id: u32,
    pub components: HashMap<TypeId, HashMap<u32, Box<dyn Any>>>, // Component Type to (Entity ID to Component)
    pub entity_components: HashMap<u32, Vec<TypeId>>, // Entity ID to Component Vectors
    // pub entities: Vec<Entity>, // TODO: Future implementation for entities
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
            components: HashMap::new(),
            entity_components: HashMap::new(),
            // entities: Vec::new(),
            player: None,
        };

        world.resources.insert(InputState::new());
        world.systems.add_system(Box::new(InputSystem::new()));

        world.player = Some(world.create_entity());

        if let Some(player) = &world.player {
            world.add_component(player.id.clone(), Transform {
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
            self.player.as_ref().map(|player| {
                if let Some(transform) = self.components.get_mut(&TypeId::of::<Transform>()).and_then(|comp_map| comp_map.get_mut(&player.id)).and_then(|comp| comp.downcast_mut::<Transform>()) {
                    transform.position.y += 1.0;
                }
            });
        }

        println!("{:#?}", self.components.get(&TypeId::of::<Transform>()).unwrap().get(&1).unwrap().downcast_ref::<Transform>());
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

    pub fn add_component<T: 'static>(&mut self, entity_id: u32, component: T) {
        let type_id = TypeId::of::<T>();

        self.components
            .entry(type_id)
            .or_insert_with(HashMap::new)
            .insert(entity_id, Box::new(component));

        self.entity_components
            .entry(entity_id)
            .or_insert_with(Vec::new)
            .push(type_id);
    }
}