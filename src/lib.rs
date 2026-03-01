//!  Unified input for Bevy
//!
//!  Mostly useful for libraries that want to support keyboard, mouse and gamepad input
//!

use bevy_ecs::prelude::*;
use bevy_input::{gamepad::GamepadButton, keyboard::KeyCode, mouse::MouseButton, prelude::*};
use std::hash::Hash;

#[derive(Debug, Clone, Default)]
pub struct InputBinding(pub Vec<InputKind>);

impl InputBinding {
    pub fn pressed_key(&self, keys: &ButtonInput<KeyCode>) -> bool {
        self.0.iter().any(|i| i.pressed_key(keys))
    }
    pub fn pressed_mouse(&self, mouse: &ButtonInput<MouseButton>) -> bool {
        self.0.iter().any(|i| i.pressed_mouse(mouse))
    }
    pub fn pressed_gamepad(&self, gamepad: &ButtonInput<GamepadButton>) -> bool {
        self.0.iter().any(|i| i.pressed_gamepad(gamepad))
    }

    pub fn just_pressed_key(&self, keys: &ButtonInput<KeyCode>) -> bool {
        self.0.iter().any(|i| i.just_pressed_key(keys))
    }
    pub fn just_pressed_mouse(&self, mouse: &ButtonInput<MouseButton>) -> bool {
        self.0.iter().any(|i| i.just_pressed_mouse(mouse))
    }
    pub fn just_pressed_gamepad(&self, gamepad: &ButtonInput<GamepadButton>) -> bool {
        self.0.iter().any(|i| i.just_pressed_gamepad(gamepad))
    }

    pub fn just_released_key(&self, keys: &ButtonInput<KeyCode>) -> bool {
        self.0.iter().any(|i| i.just_released_key(keys))
    }
    pub fn just_released_mouse(&self, mouse: &ButtonInput<MouseButton>) -> bool {
        self.0.iter().any(|i| i.just_released_mouse(mouse))
    }
    pub fn just_released_gamepad(&self, gamepad: &ButtonInput<GamepadButton>) -> bool {
        self.0.iter().any(|i| i.just_released_gamepad(gamepad))
    }

    pub fn any_pressed(
        &self,
        keys: Option<&Res<ButtonInput<KeyCode>>>,
        mouse: Option<&Res<ButtonInput<MouseButton>>>,
        gamepad: Option<&Res<ButtonInput<GamepadButton>>>,
    ) -> bool {
        self.0.iter().any(|i| i.pressed_any(keys, mouse, gamepad))
    }

    pub fn is_any_just_pressed(
        &self,
        keys: Option<&Res<ButtonInput<KeyCode>>>,
        mouse: Option<&Res<ButtonInput<MouseButton>>>,
        gamepad: Option<&Res<ButtonInput<GamepadButton>>>,
    ) -> bool {
        self.0
            .iter()
            .any(|i| i.just_pressed_any(keys, mouse, gamepad))
    }

    pub fn is_any_just_released(
        &self,
        keys: Option<&Res<ButtonInput<KeyCode>>>,
        mouse: Option<&Res<ButtonInput<MouseButton>>>,
        gamepad: Option<&Res<ButtonInput<GamepadButton>>>,
    ) -> bool {
        self.0
            .iter()
            .any(|i| i.just_released_any(keys, mouse, gamepad))
    }
}

#[derive(Component, Debug, Clone, PartialEq, Eq, Hash)]
pub enum InputKind {
    Key(KeyCode),
    Mouse(MouseButton),
    Gamepad(GamepadButton),
}

impl InputKind {
    pub fn keys(&self) -> Option<KeyCode> {
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

    pub fn pressed_key(&self, keys: &ButtonInput<KeyCode>) -> bool {
        matches!(self, InputKind::Key(k) if keys.pressed(*k))
    }

    pub fn pressed_mouse(&self, mouse: &ButtonInput<MouseButton>) -> bool {
        matches!(self, InputKind::Mouse(m) if mouse.pressed(*m))
    }

    pub fn pressed_gamepad(&self, gamepad: &ButtonInput<GamepadButton>) -> bool {
        matches!(self, InputKind::Gamepad(g) if gamepad.pressed(*g))
    }

    pub fn just_pressed_key(&self, keys: &ButtonInput<KeyCode>) -> bool {
        matches!(self, InputKind::Key(k) if keys.just_pressed(*k))
    }

    pub fn just_pressed_mouse(&self, mouse: &ButtonInput<MouseButton>) -> bool {
        matches!(self, InputKind::Mouse(m) if mouse.just_pressed(*m))
    }

    pub fn just_pressed_gamepad(&self, gamepad: &ButtonInput<GamepadButton>) -> bool {
        matches!(self, InputKind::Gamepad(g) if gamepad.just_pressed(*g))
    }

    pub fn just_released_key(&self, keys: &ButtonInput<KeyCode>) -> bool {
        matches!(self, InputKind::Key(k) if keys.just_released(*k))
    }

    pub fn just_released_mouse(&self, mouse: &ButtonInput<MouseButton>) -> bool {
        matches!(self, InputKind::Mouse(m) if mouse.just_released(*m))
    }

    pub fn just_released_gamepad(&self, gamepad: &ButtonInput<GamepadButton>) -> bool {
        matches!(self, InputKind::Gamepad(g) if gamepad.just_released(*g))
    }

    pub fn pressed_any(
        &self,
        keys: Option<&Res<ButtonInput<KeyCode>>>,
        mouse: Option<&Res<ButtonInput<MouseButton>>>,
        gamepad: Option<&Res<ButtonInput<GamepadButton>>>,
    ) -> bool {
        match self {
            InputKind::Key(k) => keys.map(|keys| keys.pressed(*k)).unwrap_or_default(),
            InputKind::Mouse(m) => mouse.map(|mouse| mouse.pressed(*m)).unwrap_or_default(),
            InputKind::Gamepad(g) => gamepad
                .map(|gamepad| gamepad.pressed(*g))
                .unwrap_or_default(),
        }
    }

    pub fn just_pressed_any(
        &self,
        keys: Option<&Res<ButtonInput<KeyCode>>>,
        mouse: Option<&Res<ButtonInput<MouseButton>>>,
        gamepad: Option<&Res<ButtonInput<GamepadButton>>>,
    ) -> bool {
        match self {
            InputKind::Key(k) => keys.map(|keys| keys.just_pressed(*k)).unwrap_or_default(),
            InputKind::Mouse(m) => mouse
                .map(|mouse| mouse.just_pressed(*m))
                .unwrap_or_default(),
            InputKind::Gamepad(g) => gamepad
                .map(|gamepad| gamepad.just_pressed(*g))
                .unwrap_or_default(),
        }
    }

    pub fn just_released_any(
        &self,
        keys: Option<&Res<ButtonInput<KeyCode>>>,
        mouse: Option<&Res<ButtonInput<MouseButton>>>,
        gamepad: Option<&Res<ButtonInput<GamepadButton>>>,
    ) -> bool {
        match self {
            InputKind::Key(k) => keys.map(|keys| keys.just_released(*k)).unwrap_or_default(),
            InputKind::Mouse(m) => mouse
                .map(|mouse| mouse.just_released(*m))
                .unwrap_or_default(),
            InputKind::Gamepad(g) => gamepad
                .map(|gamepad| gamepad.just_released(*g))
                .unwrap_or_default(),
        }
    }
}

impl From<InputKind> for InputBinding {
    fn from(value: InputKind) -> Self {
        InputBinding(vec![value])
    }
}
impl From<Vec<InputKind>> for InputBinding {
    fn from(value: Vec<InputKind>) -> Self {
        InputBinding(value)
    }
}
impl From<&[InputKind]> for InputBinding {
    fn from(value: &[InputKind]) -> Self {
        InputBinding(value.to_vec())
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

impl<const N: usize> From<[InputKind; N]> for InputBinding {
    fn from(value: [InputKind; N]) -> Self {
        Self(value.into_iter().collect())
    }
}

impl From<KeyCode> for InputBinding {
    fn from(value: KeyCode) -> Self {
        InputBinding(vec![InputKind::Key(value)])
    }
}
impl From<MouseButton> for InputBinding {
    fn from(value: MouseButton) -> Self {
        InputBinding(vec![InputKind::Mouse(value)])
    }
}
impl From<GamepadButton> for InputBinding {
    fn from(value: GamepadButton) -> Self {
        InputBinding(vec![InputKind::Gamepad(value)])
    }
}

// pub trait Binding<V> {
//     fn values(&self) -> Option<&[V]>;
// }

// pub trait IsAnyPressed<V>
// where
//     V: Copy + Eq + Hash + Send + Sync + 'static,
// {
//     fn is_any_pressed(&self, input: &ButtonInput<V>) -> bool;
// }
