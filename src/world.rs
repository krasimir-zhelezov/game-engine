use std::{any::{Any, TypeId}, collections::HashMap, sync::Arc, time::{Duration, Instant}};

use wgpu::naga::Type;
use winit::{event::{self, ElementState, KeyEvent}, keyboard::{self, Key, KeyCode, PhysicalKey}, window::Window};

use crate::{components::{camera::Camera, component_store::ComponentStore, renderable::{Color, PrimitiveType, RenderType, Renderable}, tag::Tag, transform::{Position, Scale, Transform}}, entities::{entity::Entity, entity_manager::EntityManager}, resources::resource_store::ResourceStore, systems::{camera_system::{CameraState, CameraSystem}, input_system::{self, InputState, InputSystem}, render_system::RenderSystem, system::System, system_manager::SystemManager}};

pub struct World {
    pub running: bool,
    pub last_update: Instant,
    pub accumulator: Duration,
    pub resources: ResourceStore,
    pub systems: SystemManager,
    pub next_id: u32,
    pub components: ComponentStore, // Component Type to (Entity ID to Component)
    pub entity_manager: EntityManager,
}

pub struct WorldView<'a> {
    pub resources: &'a mut ResourceStore,
    pub components: &'a mut ComponentStore,
    pub entity_manager: &'a mut EntityManager,
}

impl World {
    pub fn new(window: Arc<Window>) -> Self {
        let mut world = Self {
            running: true,
            last_update: Instant::now(),
            accumulator: Duration::ZERO,
            resources: ResourceStore::new(),
            systems: SystemManager::new(),
            next_id: 0,
            components: ComponentStore::new(),
            entity_manager: EntityManager::new(),
        };

        //world.resources.insert(InputState::new());
        //world.resources.insert(CameraState::new());
        //world.systems.add_system(Box::new(CameraSystem::new()));
        //world.systems.add_system(Box::new(InputSystem::new()));
        //world.systems.add_system(Box::new(pollster::block_on(RenderSystem::new(window))));
        

        let player_id = world.entity_manager.create_entity();
        world.components.add_component::<Transform>(player_id, Transform {
            position: Position { x: 0.0, y: 0.0 },
            scale: Scale { x: 1.0, y: 1.0 },
            rotation: 1.0,
        });
        world.components.add_component(player_id, Tag::new("Player"));
        // world.components.add_component(player, Renderable {
        //     color: Color { r: 255.0, g: 0.0, b: 0.0, a: 1.0 },
        //     render_type: RenderType::Primitive {
        //         primitive_type: PrimitiveType::Rectangle,
        //         parameters: [0.0, 0.0, 0.0, 0.0],
        //     },
        //     visible: true,
        // });

        let enemy_id = world.entity_manager.create_entity();
        world.components.add_component::<Transform>(enemy_id, Transform {
            position: Position { x: 5.0, y: 5.0 },
            scale: Scale { x: 1.0, y: 1.0 },
            rotation: 0.0,
        });

        let camera_id = world.entity_manager.create_entity();

        println!("{:?}", (player_id, enemy_id, camera_id));
        // world.components.add_component(camera, Transform {
        //     position: Position { x: 0.0, y: 0.0 },
        //     scale: Scale { x: 1.0, y: 1.0 },
        //     rotation: 0.0,
        // });
        // world.components.add_component(camera, Camera {
        //     zoom: 10.0, // Increase zoom to see more
        //     aspect_ratio: 16.0 / 9.0, // Set proper aspect ratio
        //     near_plane: -100.0,
        //     far_plane: 100.0,
        //     fov: 1.0, // Not used in orthographic
        // });

        world
    }

    pub fn update(&mut self) {
        // self.systems.update(&mut WorldView {
        //     resources: &mut self.resources,
        //     components: &mut self.components,
        //     entity_manager: &mut self.entity_manager,
        // });

        // let input = self.resources.get::<InputState>().unwrap();
        // if input.is_key_pressed(KeyCode::KeyW) {
        //     // let entity = self.entities.get_entities_by_tag("Player", &self.components)[0];

        //     // if let Some(transform) = self.components.get_component_mut::<Transform>(&entity) {
        //     //     transform.position.y += 1.0;
        //     // }
        // }
    }

    pub fn render(&self) {
        // graphics.draw(&mut []);
    }

    pub fn handle_keyboard_input(&mut self, event: &KeyEvent) {
        let Self { resources, systems, .. } = self;

        let input_system = systems.get_system_mut::<InputSystem>().unwrap();

        let mut view = WorldView {
            resources,
            components: &mut self.components,
            entity_manager: &mut self.entity_manager,
        };

        input_system.handle_keyboard_input(&mut view, event);
    }

    pub fn handle_mouse_input(&self) {
        todo!();
    }
}