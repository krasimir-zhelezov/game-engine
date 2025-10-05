use std::collections::HashMap;

use winit::{event::{ElementState, KeyEvent}, keyboard::PhysicalKey};

use crate::{systems::system::System, world::{World, WorldView}};
use winit::keyboard::KeyCode;

pub struct InputState {
    pub keys: HashMap<KeyCode, bool>,
}

impl InputState {
    pub fn new() -> Self {
        Self {
            keys: HashMap::new(),
        }
    }

    pub fn is_key_pressed(&self, key: KeyCode) -> bool {
        *self.keys.get(&key).unwrap_or(&false)
    }
}

pub struct InputSystem;

impl InputSystem {
    pub fn new() -> Self {
        Self
    }

    pub fn handle_keyboard_input(&mut self, world: &mut WorldView, event: &KeyEvent) {
        match event.state {
            ElementState::Pressed => {
                if let PhysicalKey::Code(key_code) = event.physical_key {
                    let input_state = world.resources.get_mut::<InputState>().unwrap();
                    input_state.keys.insert(key_code, true);
                    // println!("Key pressed: {:?}", key_code);
                }
            }
            ElementState::Released => {
                if let PhysicalKey::Code(key_code) = event.physical_key {
                    let input_state = world.resources.get_mut::<InputState>().unwrap();
                    input_state.keys.remove(&key_code);
                    // println!("Key pressed: {:?}", key_code);
                }
            }
        }
    }
}

impl System for InputSystem {
    fn update(&mut self, world: &mut WorldView) {
        
    }
}