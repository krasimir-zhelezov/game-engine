use std::{
    any::{Any, TypeId},
    collections::HashMap,
    sync::Arc,
    time::{Duration, Instant},
};

use wgpu::naga::Type;
use winit::{
    event::{self, ElementState, KeyEvent},
    keyboard::{self, Key, KeyCode, PhysicalKey},
    window::Window,
};

use crate::{
    components::{
        camera::Camera, collider::{Collider, ColliderShape}, component_store::ComponentStore, custom::player_controller::PlayerController, renderable::{Color, PrimitiveType, RenderType, Renderable}, tag::Tag, transform::{Position, Scale, Transform}, velocity::Velocity
    },
    entities::entity_manager::EntityManager,
    resources::{asset_manager::{self, AssetManager}, collision_events::{self, CollisionEvents}, resource_store::ResourceStore},
    systems::{
        camera_system::{CameraState, CameraSystem}, collision_system::CollisionSystem, custom::{player_movement_system::PlayerMovementSystem, stress_test_system::StressTestSystem}, input_system::{self, InputState, InputSystem}, render_system::RenderSystem, system::System, system_manager::SystemManager, velocity_system::VelocitySystem
    },
};

pub struct World {
    pub running: bool,
    pub last_update: Instant,
    pub accumulator: Duration,
    pub resources: ResourceStore,
    pub systems: SystemManager,
    pub next_id: u32,
    pub components: ComponentStore, // Component Type to (Entity ID to Component)
    pub entity_manager: EntityManager,
    pub window: Arc<Window>,

    pub fps: u32,
    frame_count: u32,
    fps_timer: Instant,
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
            window: window.clone(),

            fps: 0,
            frame_count: 0,
            fps_timer: Instant::now(),
        };
        
        let asset_manager = AssetManager::new();

        world.resources.insert(InputState::new());
        world.resources.insert(CameraState::new());
        world.resources.insert(CollisionEvents::default());

        world.systems.add_system(Box::new(CameraSystem::new()));
        world.systems.add_system(Box::new(PlayerMovementSystem::new()));
        world.systems.add_system(Box::new(pollster::block_on(RenderSystem::new(window))));
        world.systems.add_system(Box::new(InputSystem::new()));
        world.systems.add_system(Box::new(VelocitySystem::new()));
        world.systems.add_system(Box::new(CollisionSystem::new()));
        // world.systems.add_system(Box::new(StressTestSystem::new(100)));

        let camera_id = world.entity_manager.create_entity();
        world.components.add_component(camera_id, Transform {
            position: Position { x: 0.0, y: 0.0 },
            scale: Scale { x: 1.0, y: 1.0 },
            rotation: 0.0,
        });
        world.components.add_component(camera_id, Camera {
            zoom: 10.0, // Increase zoom to see more
            aspect_ratio: 4.0 / 3.0, // 16.0 / 9.0,
            near_plane: -100.0,
            far_plane: 100.0,
            fov: 1.0, // Not used in orthographic
        });

        let player_id = world.entity_manager.create_entity();
        world.components.add_component::<Transform>(
            player_id,
            Transform {
                position: Position { x: 0.0, y: 0.0 },
                scale: Scale { x: 2.0, y: 2.0 },
                rotation: 1.0,
            },
        );
        world.components.add_component(player_id, Renderable::new_texture(asset_manager.get_texture("player.png").unwrap()));
        world.components.add_component(player_id, PlayerController {
            movement_speed: 0.5,
        });
        world.components.add_component(player_id, Collider {
            shape: ColliderShape::Box { width: 1.0, height: 1.0 },
        });
        
        let entity1_id = world.entity_manager.create_entity();
        world.components.add_component::<Transform>(entity1_id, Transform {
            position: Position { x: 0.0, y: 0.0 },
            scale: Scale { x: 2.0, y: 2.0 },
            rotation: 1.0,
        });
        world.components.add_component::<Renderable>(entity1_id, Renderable::new_rectangle(
            Color::from_rgba8(255.0, 205.0, 30.0, 255.0),
            10.0,
            10.0
        ));
        // world.components.add_component::<Velocity>(entity1_id, Velocity { x: 0.0, y: 0.01 });
        world.components.add_component(entity1_id, Collider {
            shape: ColliderShape::Box { width: 1.0, height: 1.0 },
        });

        world
    }

    pub fn update(&mut self) {
        self.systems.update(&mut WorldView {
            resources: &mut self.resources,
            components: &mut self.components,
            entity_manager: &mut self.entity_manager,
        });

        self.frame_count += 1;

        if self.fps_timer.elapsed() >= Duration::from_secs(1) {
            self.fps = self.frame_count;
            self.frame_count = 0;
            self.fps_timer = Instant::now();
        }

        let title = format!("Skalora 2D Game Engine | FPS: {} | Entities: {} | Collisions: {}", self.fps, self.entity_manager.entity_count, self.resources.get::<CollisionEvents>().unwrap().events.len());

        self.window.set_title(&title);

        if let Some(collision_events) = self.resources.get_mut::<CollisionEvents>() {
            collision_events.events.clear();
        }

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
        let Self {
            resources, systems, ..
        } = self;

        let input_system = systems.get_system_mut::<InputSystem>().unwrap();

        let mut view = WorldView {
            resources,
            components: &mut self.components,
            entity_manager: &mut self.entity_manager,
        };

        input_system.handle_keyboard_input(&mut view, event);
    }

    pub fn delete_entity(&mut self, entity_id: u32) {
        self.entity_manager.delete_entity(entity_id);

        self.components.remove_entity(entity_id);
    }

    pub fn handle_mouse_button(&mut self, state: ElementState, button: winit::event::MouseButton) {
        let input_system = self.systems.get_system_mut::<InputSystem>().unwrap();

        let mut view = WorldView {
            resources: &mut self.resources,
            components: &mut self.components,
            entity_manager: &mut self.entity_manager,
        };

        input_system.handle_mouse_button(&mut view, state, button);
    }

    pub fn handle_cursor_moved(&mut self, position: winit::dpi::PhysicalPosition<f64>) {
        let input_system = self.systems.get_system_mut::<InputSystem>().unwrap();

        let mut view = WorldView {
            resources: &mut self.resources,
            components: &mut self.components,
            entity_manager: &mut self.entity_manager,
        };

        input_system.handle_cursor_moved(&mut view, position);
    }

    pub fn handle_mouse_wheel(&mut self, delta: winit::event::MouseScrollDelta) {
        let input_system = self.systems.get_system_mut::<InputSystem>().unwrap();

        let mut view = WorldView {
            resources: &mut self.resources,
            components: &mut self.components,
            entity_manager: &mut self.entity_manager,
        };

        input_system.handle_mouse_wheel(&mut view, delta);
    }

    pub fn handle_mouse_motion(&mut self, delta: (f64, f64)) {
        let input_system = self.systems.get_system_mut::<InputSystem>().unwrap();

        let mut view = WorldView {
            resources: &mut self.resources,
            components: &mut self.components,
            entity_manager: &mut self.entity_manager,
        };

        input_system.handle_mouse_motion(&mut view, delta);
    }
}