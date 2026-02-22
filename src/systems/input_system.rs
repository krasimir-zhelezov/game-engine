use std::collections::{HashMap, HashSet};

use winit::{event::{ElementState, KeyEvent}, keyboard::{Key, PhysicalKey}};

use crate::{systems::system::System, world::{World, WorldView}};
use winit::keyboard::KeyCode;

pub struct InputState {
    pub held_keys: HashSet<KeyCode>,
    pub just_pressed: HashSet<KeyCode>,
    pub just_released: HashSet<KeyCode>,
}

impl InputState {
    pub fn new() -> Self {
        Self {
            held_keys: HashSet::new(),
            just_pressed: HashSet::new(),
            just_released: HashSet::new(),
        }
    }

    pub fn is_key_held(&self, key: KeyCode) -> bool {
        self.held_keys.contains(&key)
    }

    pub fn is_key_just_pressed(&self, key: KeyCode) -> bool {
        self.just_pressed.contains(&key)
    }

    pub fn is_key_just_released(&self, key: KeyCode) -> bool {
        self.just_released.contains(&key)
    }
}

pub struct InputSystem;

impl InputSystem {
    pub fn new() -> Self {
        Self
    }

    pub fn handle_keyboard_input(&mut self, world: &mut WorldView, event: &KeyEvent) {
        let mut input_state = world.resources.get_mut::<InputState>().unwrap();
        
        match event.state {
            ElementState::Pressed => {
                if let PhysicalKey::Code(key_code) = event.physical_key {
                    if input_state.held_keys.insert(key_code) {
                        input_state.just_pressed.insert(key_code);
                    }

                    println!("Key pressed: {:#?}", key_code);
                }
            }
            ElementState::Released => {
                if let PhysicalKey::Code(key_code) = event.physical_key {
                    
                    input_state.held_keys.remove(&key_code);
                    input_state.just_released.insert(key_code);

                    println!("Key released: {:#?}", key_code);
                }
            }
        }
    }
}

impl System for InputSystem {
    fn update(&mut self, world: &mut WorldView) {
        let mut input_state = world.resources.get_mut::<InputState>().unwrap();

        input_state.just_pressed.clear();
        input_state.just_released.clear();
    }
}