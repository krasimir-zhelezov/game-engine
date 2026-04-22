use std::{
    sync::Arc, time::{Duration, Instant}
};

use winit::{
    event::{ElementState, KeyEvent},
    window::Window,
};

use crate::{
    components::{
        camera::Camera, collider::Collider, component_store::ComponentStore, renderable::Renderable, tag::Tag, transform::{Position, Scale, Transform}, velocity::Velocity
    },
    entities::entity_manager::EntityManager,
    resources::{asset_manager::AssetManager, collision_events::CollisionEvents, resource_store::ResourceStore},
    systems::{
        camera_system::{CameraState, CameraSystem}, collision_system::CollisionSystem, input_system::{InputState, InputSystem}, system_manager::SystemManager, velocity_system::VelocitySystem
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
    pub window: Option<Arc<Window>>,

    pub fps: u32,
    frame_count: u32,
    fps_timer: Instant,

    pub window_title: String,
    pub show_debug_title: bool,
}

pub struct WorldView<'a> {
    pub resources: &'a mut ResourceStore,
    pub components: &'a mut ComponentStore,
    pub entity_manager: &'a mut EntityManager,
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
            entity_manager: EntityManager::new(),
            window: None,

            fps: 0,
            frame_count: 0,
            fps_timer: Instant::now(),

            window_title: "Skalora 2D Game Engine".to_string(),
            show_debug_title: true,
        };
        
        let asset_manager = AssetManager::new();

        world.resources.insert(asset_manager);
        world.resources.insert(InputState::new());
        world.resources.insert(CameraState::new());
        world.resources.insert(CollisionEvents::default());

        world.components.register_component::<Transform>();
        world.components.register_component::<Camera>();
        world.components.register_component::<Renderable>();
        world.components.register_component::<Collider>();
        world.components.register_component::<Velocity>();
        world.components.register_component::<Tag>();

        world.systems.add_system(Box::new(CameraSystem::new()));
        world.systems.add_system(Box::new(InputSystem::new()));
        world.systems.add_system(Box::new(VelocitySystem::new()));
        world.systems.add_system(Box::new(CollisionSystem::new()));
        
        world
    }

    pub fn init_graphics(&mut self, window: Arc<Window>) {
        self.window = Some(window.clone());
        self.systems.add_system(Box::new(pollster::block_on(crate::systems::render_system::RenderSystem::new(window))));
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

        if self.show_debug_title {
            // Using map_or is a slightly safer alternative to unwrap() here 
            // just in case the CollisionEvents resource gets accidentally removed
            let collision_count = self.resources.get::<CollisionEvents>().map_or(0, |c| c.events.len());
            
            let title = format!(
                "{} | FPS: {} | Entities: {} | Collisions: {}", 
                self.window_title, 
                self.fps, 
                self.entity_manager.entity_count, 
                collision_count
            );

            if let Some(window) = &self.window {
                window.set_title(&title);
            }
        }

        if let Some(collision_events) = self.resources.get_mut::<CollisionEvents>() {
            collision_events.events.clear();
        }
    }

    pub fn render(&self) {
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

    pub fn spawn_camera(&mut self) -> u32 {
        let camera_id = self.entity_manager.create_entity();
        
        self.components.add_component(
            camera_id,
            Transform {
                position: Position { x: 0.0, y: 0.0 },
                scale: Scale { x: 1.0, y: 1.0 },
                rotation: 0.0,
            },
        );
        
        self.components.add_component(
            camera_id,
            Camera {
                zoom: 10.0,
                aspect_ratio: 4.0 / 3.0,
                near_plane: -100.0,
                far_plane: 100.0,
                fov: 1.0,
            },
        );

        camera_id
    }

    pub fn set_window_title(&mut self, title: &str) {
        self.window_title = title.to_string();
        
        if !self.show_debug_title {
            if let Some(window) = &self.window {
                window.set_title(&self.window_title);
            }
        }
    }

    pub fn set_debug_title(&mut self, enabled: bool) {
        self.show_debug_title = enabled;
        
        if !enabled {
            if let Some(window) = &self.window {
                window.set_title(&self.window_title);
            }
        }
    }
}