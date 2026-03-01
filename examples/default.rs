use bevy::prelude::*;
use bevy_top_down_camera::*;
use bevy_unified_input::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TopDownCameraPlugin))
        .add_systems(
            Startup,
            (spawn_camera, spawn_ui, spawn_world, spawn_player).chain(),
        )
        .add_systems(Update, actions)
        .run();
}

#[derive(Component)]
struct Player;

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Player,
        TopDownCameraTarget,
        Mesh3d(meshes.add(Capsule3d::new(0.5, 1.0))),
        MeshMaterial3d(materials.add(Color::srgba(0.9, 0.9, 0.9, 0.5))),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        IsDefaultUiCamera,
        Camera::default(),
        Camera3d::default(),
        TopDownCamera::default(),
    ));
}

#[derive(Resource)]
struct MyInput {
    forward: InputBinding,
    backward: InputBinding,
    left: InputBinding,
    right: InputBinding,
    follow_camera: InputBinding,
}

// This is purely for demonstration purposes,
// obviously it is not the best way to do it if you want the only input
impl Default for MyInput {
    fn default() -> Self {
        Self {
            forward: [KeyCode::KeyW.into(), GamepadButton::DPadUp.into()].into(),
            backward: [KeyCode::KeyS.into(), GamepadButton::DPadDown.into()].into(),
            left: [KeyCode::KeyA.into(), GamepadButton::DPadLeft.into()].into(),
            right: [KeyCode::KeyD.into(), GamepadButton::DPadRight.into()].into(),
            follow_camera: [KeyCode::KeyF.into(), GamepadButton::Start.into()].into(),
        }
    }
}

fn spawn_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.insert_resource(MyInput::default());

    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(50.0, 50.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.11, 0.27, 0.16))),
    ));

    commands.spawn((
        PointLight {
            intensity: 1500.0 * 1000.0,
            ..default()
        },
        Transform::from_xyz(0.0, 5.0, 0.0),
    ));
}

fn spawn_ui(mut commands: Commands) {
    commands.spawn((
        Node {
            width: Val::Percent(40.0),
            height: Val::Percent(20.0),
            align_items: AlignItems::Start,
            position_type: PositionType::Absolute,
            left: Val::Px(0.0),
            top: Val::Px(0.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Start,
            ..default()
        },
        children![
            (Node::default(), Text("G - toggle mode".to_string())),
            (
                Node::default(),
                Text("F - toggle follow player mode".to_string())
            ),
            (
                Node::default(),
                Text("Right mouse - hold to rotate camera horizontally".to_string())
            ),
        ],
    ));
}

fn actions(
    time: Res<Time>,
    keys: Option<Res<ButtonInput<KeyCode>>>,
    gamepad: Option<Res<ButtonInput<GamepadButton>>>,
    my_input: Res<MyInput>,
    mut cam_q: Query<&mut TopDownCamera>,
    mut player_q: Query<&mut Transform, With<Player>>,
) {
    let mut direction = Vec3::ZERO;

    let (forward, backward, left, right) = (
        my_input
            .forward
            .pressed_any(keys.as_ref(), None, gamepad.as_ref()),
        my_input
            .backward
            .pressed_any(keys.as_ref(), None, gamepad.as_ref()),
        my_input
            .left
            .pressed_any(keys.as_ref(), None, gamepad.as_ref()),
        my_input
            .right
            .pressed_any(keys.as_ref(), None, gamepad.as_ref()),
    );

    if forward {
        direction.z -= 1.0;
    }
    if backward {
        direction.z += 1.0;
    }
    if left {
        direction.x -= 1.0;
    }
    if right {
        direction.x += 1.0;
    }

    let follow = keys
        .map(|keys| my_input.follow_camera.just_pressed_key(&keys))
        .unwrap_or_default();

    if follow {
        let Ok(mut cam) = cam_q.single_mut() else {
            return;
        };
        cam.motion.follow = !cam.motion.follow;
    }

    if direction.length_squared() > 0.0 {
        direction = direction.normalize();
        let speed = 10.0;
        for mut transform in &mut player_q {
            transform.translation += direction * speed * time.delta_secs();
        }
    }
}
