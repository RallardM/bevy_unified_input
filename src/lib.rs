use bevy::{
    input::{gamepad::GamepadButton, keyboard::KeyCode, mouse::MouseButton},
    prelude::*,
};

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InputKind {
    Key(KeyCode),
    Mouse(MouseButton),
    Gamepad(GamepadButton),
}
impl InputKind {
    pub fn is_key_pressed(&self, keys: &ButtonInput<KeyCode>) -> bool {
        self.key().map(|key| keys.pressed(key)).unwrap_or_default()
    }

    pub fn is_mouse_pressed(&self, mouse: &ButtonInput<MouseButton>) -> bool {
        self.mouse()
            .map(|btn| mouse.pressed(btn))
            .unwrap_or_default()
    }

    pub fn is_gamepad_pressed(&self, gamepad: &ButtonInput<GamepadButton>) -> bool {
        self.gamepad()
            .map(|btn| gamepad.pressed(btn))
            .unwrap_or_default()
    }

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
