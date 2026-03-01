//!  Unified input for Bevy
//!
//!  Mostly useful for libraries that want to support keyboard, mouse and gamepad input
//!

use bevy_ecs::prelude::*;
use bevy_input::{gamepad::GamepadButton, keyboard::KeyCode, mouse::MouseButton, prelude::*};
use std::hash::Hash;

pub trait IsPressed<V: Sync + Send + Hash + Eq + Clone> {
    fn is_pressed(&self, value: ButtonInput<V>) -> bool;
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InputKind {
    Key(KeyCode),
    Mouse(MouseButton),
    Gamepad(GamepadButton),
}

impl InputKind {
    pub fn key(&self) -> Option<KeyCode> {
        match self {
            InputKind::Key(key) => Some(*key),
            _ => None,
        }
    }

    pub fn mouse(&self) -> Option<MouseButton> {
        match self {
            InputKind::Mouse(btn) => Some(*btn),
            _ => None,
        }
    }

    pub fn gamepad(&self) -> Option<GamepadButton> {
        match self {
            InputKind::Gamepad(btn) => Some(*btn),
            _ => None,
        }
    }
}

impl IsPressed<KeyCode> for InputKind {
    fn is_pressed(&self, value: ButtonInput<KeyCode>) -> bool {
        self.key().map(|key| value.pressed(key)).unwrap_or_default()
    }
}

impl IsPressed<MouseButton> for InputKind {
    fn is_pressed(&self, value: ButtonInput<MouseButton>) -> bool {
        self.mouse()
            .map(|btn| value.pressed(btn))
            .unwrap_or_default()
    }
}

impl IsPressed<GamepadButton> for InputKind {
    fn is_pressed(&self, value: ButtonInput<GamepadButton>) -> bool {
        self.gamepad()
            .map(|btn| value.pressed(btn))
            .unwrap_or_default()
    }
}

impl From<KeyCode> for InputKind {
    fn from(value: KeyCode) -> Self {
        InputKind::Key(value)
    }
}
impl From<MouseButton> for InputKind {
    fn from(value: MouseButton) -> Self {
        InputKind::Mouse(value)
    }
}

impl From<GamepadButton> for InputKind {
    fn from(value: GamepadButton) -> Self {
        InputKind::Gamepad(value)
    }
}
