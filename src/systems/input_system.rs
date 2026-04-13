use std::collections::{HashSet};

use winit::{event::{ElementState, KeyEvent, MouseButton}, keyboard::{PhysicalKey}};

use crate::{systems::system::System, world::{WorldView}};
use winit::keyboard::KeyCode;

pub struct InputState {
    pub held_keys: HashSet<KeyCode>,
    pub just_pressed_keys: HashSet<KeyCode>,
    pub just_released_keys: HashSet<KeyCode>,

    pub held_mouse_buttons: HashSet<MouseButton>,
    pub just_pressed_mouse_buttons: HashSet<MouseButton>,
    pub just_released_mouse_buttons: HashSet<MouseButton>,

    pub mouse_position: (f64, f64),
    pub mouse_delta: (f64, f64),
    pub scroll_delta: (f32, f32),
}

impl InputState {
    pub fn new() -> Self {
        Self {
            held_keys: HashSet::new(),
            just_pressed_keys: HashSet::new(),
            just_released_keys: HashSet::new(),

            held_mouse_buttons: HashSet::new(),
            just_pressed_mouse_buttons: HashSet::new(),
            just_released_mouse_buttons: HashSet::new(),

            mouse_position: (0.0, 0.0),
            mouse_delta: (0.0, 0.0),
            scroll_delta: (0.0, 0.0),
        }
    }

    pub fn is_key_held(&self, key: KeyCode) -> bool {
        self.held_keys.contains(&key)
    }

    pub fn is_key_just_pressed(&self, key: KeyCode) -> bool {
        self.just_pressed_keys.contains(&key)
    }

    pub fn is_key_just_released(&self, key: KeyCode) -> bool {
        self.just_released_keys.contains(&key)
    }

    pub fn is_mouse_button_held(&self, button: MouseButton) -> bool {
        self.held_mouse_buttons.contains(&button)
    }
    pub fn is_mouse_button_just_pressed(&self, button: MouseButton) -> bool {
        self.just_pressed_mouse_buttons.contains(&button)
    }
    pub fn is_mouse_button_just_released(&self, button: MouseButton) -> bool {
        self.just_released_mouse_buttons.contains(&button)
    }
}

pub struct InputSystem;

impl InputSystem {
    pub fn new() -> Self {
        Self
    }

    pub fn handle_keyboard_input(&mut self, world: &mut WorldView, event: &KeyEvent) {
        let input_state = world.resources.get_mut::<InputState>().unwrap();
        
        match event.state {
            ElementState::Pressed => {
                if let PhysicalKey::Code(key_code) = event.physical_key {
                    if input_state.held_keys.insert(key_code) {
                        input_state.just_pressed_keys.insert(key_code);
                    }

                    println!("Key pressed: {:#?}", key_code);
                }
            }
            ElementState::Released => {
                if let PhysicalKey::Code(key_code) = event.physical_key {
                    
                    input_state.held_keys.remove(&key_code);
                    input_state.just_released_keys.insert(key_code);

                    println!("Key released: {:#?}", key_code);
                }
            }
        }
    }

    pub fn handle_mouse_button(&mut self, world: &mut WorldView, state: ElementState, button: MouseButton) {
        let input_state = world.resources.get_mut::<InputState>().unwrap();

        match state {
            ElementState::Pressed => {
                if input_state.held_mouse_buttons.insert(button) {
                    input_state.just_pressed_mouse_buttons.insert(button);
                }
                // println!("Mouse button pressed: {:?}", button);
            }
            ElementState::Released => {
                input_state.held_mouse_buttons.remove(&button);
                input_state.just_released_mouse_buttons.insert(button);
                // println!("Mouse button released: {:?}", button);
            }
        }
    }

    pub fn handle_cursor_moved(&mut self, world: &mut WorldView, position: winit::dpi::PhysicalPosition<f64>) {
        let input_state = world.resources.get_mut::<InputState>().unwrap();
        input_state.mouse_position = (position.x, position.y);
        // println!("Cursor moved to: X: {:.1}, Y: {:.1}", position.x, position.y);
    }

    pub fn handle_mouse_motion(&mut self, world: &mut WorldView, delta: (f64, f64)) {
        let input_state = world.resources.get_mut::<InputState>().unwrap();
        input_state.mouse_delta.0 += delta.0;
        input_state.mouse_delta.1 += delta.1;
        println!("Raw mouse motion delta: X: {:.1}, Y: {:.1}", delta.0, delta.1);
    }

    pub fn handle_mouse_wheel(&mut self, world: &mut WorldView, delta: winit::event::MouseScrollDelta) {
        let input_state = world.resources.get_mut::<InputState>().unwrap();
        
        match delta {
            winit::event::MouseScrollDelta::LineDelta(x, y) => {
                input_state.scroll_delta.0 += x;
                input_state.scroll_delta.1 += y;
                // println!("Mouse wheel scrolled (Lines): X: {}, Y: {}", x, y);
            }
            winit::event::MouseScrollDelta::PixelDelta(pos) => {
                input_state.scroll_delta.0 += pos.x as f32 * 0.1;
                input_state.scroll_delta.1 += pos.y as f32 * 0.1;
                // println!("Mouse wheel scrolled (Pixels): X: {}, Y: {}", pos.x, pos.y);
            }
        }
    }
}

impl System for InputSystem {
    fn update(&mut self, world: &mut WorldView) {
        let input_state = world.resources.get_mut::<InputState>().unwrap();

        input_state.just_pressed_keys.clear();
        input_state.just_released_keys.clear();

        input_state.just_pressed_mouse_buttons.clear();
        input_state.just_released_mouse_buttons.clear();

        input_state.mouse_delta = (0.0, 0.0);
        input_state.scroll_delta = (0.0, 0.0);
    }
}