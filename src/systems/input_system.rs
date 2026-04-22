use std::collections::HashSet;

use winit::{
    event::{ElementState, KeyEvent, MouseButton},
    keyboard::PhysicalKey,
};

use crate::{systems::system::System, world::WorldView};
use winit::keyboard::KeyCode;

/// Stores the current state of user input, including keyboard and mouse activity.
///
/// This struct is designed to be stored as a resource in the `WorldView`. It provides
/// an easy way for other systems to query the current state of inputs (e.g., checking
/// if a jump key was just pressed or if the mouse moved during the current frame).
pub struct InputState {
    /// Keys that are currently being held down.
    pub held_keys: HashSet<KeyCode>,
    /// Keys that transitioned from released to pressed during the current frame.
    pub just_pressed_keys: HashSet<KeyCode>,
    /// Keys that transitioned from pressed to released during the current frame.
    pub just_released_keys: HashSet<KeyCode>,

    /// Mouse buttons that are currently being held down.
    pub held_mouse_buttons: HashSet<MouseButton>,
    /// Mouse buttons that transitioned from released to pressed during the current frame.
    pub just_pressed_mouse_buttons: HashSet<MouseButton>,
    /// Mouse buttons that transitioned from pressed to released during the current frame.
    pub just_released_mouse_buttons: HashSet<MouseButton>,

    /// The current absolute position of the mouse cursor within the window.
    pub mouse_position: (f64, f64),
    /// The relative movement of the mouse cursor since the last frame.
    pub mouse_delta: (f64, f64),
    /// The accumulated scroll wheel delta since the last frame.
    pub scroll_delta: (f32, f32),
}

impl InputState {
    /// Creates a new, empty `InputState` with all inputs released and deltas zeroed.
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

    /// Returns `true` if the specified key is currently being held down.
    pub fn is_key_held(&self, key: KeyCode) -> bool {
        self.held_keys.contains(&key)
    }

    /// Returns `true` if the specified key was pressed during the current frame.
    pub fn is_key_just_pressed(&self, key: KeyCode) -> bool {
        self.just_pressed_keys.contains(&key)
    }

    /// Returns `true` if the specified key was released during the current frame.
    pub fn is_key_just_released(&self, key: KeyCode) -> bool {
        self.just_released_keys.contains(&key)
    }

    /// Returns `true` if the specified mouse button is currently being held down.
    pub fn is_mouse_button_held(&self, button: MouseButton) -> bool {
        self.held_mouse_buttons.contains(&button)
    }

    /// Returns `true` if the specified mouse button was pressed during the current frame.
    pub fn is_mouse_button_just_pressed(&self, button: MouseButton) -> bool {
        self.just_pressed_mouse_buttons.contains(&button)
    }

    /// Returns `true` if the specified mouse button was released during the current frame.
    pub fn is_mouse_button_just_released(&self, button: MouseButton) -> bool {
        self.just_released_mouse_buttons.contains(&button)
    }
}

/// A system responsible for intercepting window events and updating the `InputState`.
///
/// The `InputSystem` processes raw events from `winit` (such as keyboard presses,
/// mouse movement, and scrolling) and translates them into the persistent and
/// per-frame data stored in the `InputState` resource.
pub struct InputSystem;

impl InputSystem {
    /// Creates a new `InputSystem`.
    pub fn new() -> Self {
        Self
    }

    /// Processes a raw keyboard event from `winit` and updates the internal `InputState`.
    ///
    /// Tracks `held_keys`, `just_pressed_keys`, and `just_released_keys` based on the
    /// physical key code.
    pub fn handle_keyboard_input(&mut self, world: &mut WorldView, event: &KeyEvent) {
        let input_state = world.resources.get_mut::<InputState>().unwrap();

        match event.state {
            ElementState::Pressed => {
                if let PhysicalKey::Code(key_code) = event.physical_key {
                    if input_state.held_keys.insert(key_code) {
                        input_state.just_pressed_keys.insert(key_code);
                    }
                }
            }
            ElementState::Released => {
                if let PhysicalKey::Code(key_code) = event.physical_key {
                    input_state.held_keys.remove(&key_code);
                    input_state.just_released_keys.insert(key_code);
                }
            }
        }
    }

    /// Processes a raw mouse button event from `winit` and updates the internal `InputState`.
    pub fn handle_mouse_button(
        &mut self,
        world: &mut WorldView,
        state: ElementState,
        button: MouseButton,
    ) {
        let input_state = world.resources.get_mut::<InputState>().unwrap();

        match state {
            ElementState::Pressed => {
                if input_state.held_mouse_buttons.insert(button) {
                    input_state.just_pressed_mouse_buttons.insert(button);
                }
            }
            ElementState::Released => {
                input_state.held_mouse_buttons.remove(&button);
                input_state.just_released_mouse_buttons.insert(button);
            }
        }
    }

    /// Updates the absolute cursor position within the `InputState`.
    pub fn handle_cursor_moved(
        &mut self,
        world: &mut WorldView,
        position: winit::dpi::PhysicalPosition<f64>,
    ) {
        let input_state = world.resources.get_mut::<InputState>().unwrap();
        input_state.mouse_position = (position.x, position.y);
    }

    /// Accumulates raw mouse motion (often hardware-level delta) into the `InputState`.
    ///
    /// This is particularly useful for first-person cameras where absolute cursor
    /// position is locked or irrelevant.
    pub fn handle_mouse_motion(&mut self, world: &mut WorldView, delta: (f64, f64)) {
        let input_state = world.resources.get_mut::<InputState>().unwrap();
        input_state.mouse_delta.0 += delta.0;
        input_state.mouse_delta.1 += delta.1;
    }

    /// Accumulates mouse scroll wheel delta into the `InputState`.
    ///
    /// Normalizes both line-based scrolling (standard mice) and pixel-based
    /// scrolling (trackpads) into a unified delta value.
    pub fn handle_mouse_wheel(
        &mut self,
        world: &mut WorldView,
        delta: winit::event::MouseScrollDelta,
    ) {
        let input_state = world.resources.get_mut::<InputState>().unwrap();

        match delta {
            winit::event::MouseScrollDelta::LineDelta(x, y) => {
                input_state.scroll_delta.0 += x;
                input_state.scroll_delta.1 += y;
            }
            winit::event::MouseScrollDelta::PixelDelta(pos) => {
                input_state.scroll_delta.0 += pos.x as f32 * 0.1;
                input_state.scroll_delta.1 += pos.y as f32 * 0.1;
            }
        }
    }
}

impl System for InputSystem {
    /// Performs end-of-frame (or start-of-frame) cleanup for the `InputState`.
    ///
    /// This clears all per-frame data, such as "just pressed/released" states
    /// and accumulated deltas for mouse movement and scrolling, ensuring they
    /// don't accidentally carry over to the next frame.
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