use bevy::prelude::*;
use bevy_flycam::PlayerPlugin;
use bevy::window::WindowMode::Fullscreen;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_mod_picking::*;
pub const HEIGHT: f32 = 720.0;
pub const WIDTH: f32 = 1280.0;

fn spawn_fruit(mut commands: Commands, assets: Res<AssetServer>) {
        let fruit = assets.load("terrain.glb#Scene0");
    commands.spawn_bundle(SceneBundle {
        scene: fruit,
        transform: Transform::from_xyz(1.0, 1.0, 1.0),
        ..Default::default()
    });
}

fn spawn_camera(mut commands: Commands) {
    commands
        .spawn_bundle(Camera3dBundle {
            transform: Transform::from_xyz(4.0, 8.0, 4.0)
                .with_rotation(Quat::from_rotation_y(-1.5)),
            ..default()
        })
        .insert_bundle(PickingCameraBundle::default());
}

//contains ground and a cube
fn spawn_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(1.0, 0.5, 1.0).into()),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        })
        .insert(Name::new("player")).insert_bundle(PickableBundle::default());
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 10.0 })),
            material: materials.add(Color::rgb(0.1, 0.5, 1.0).into()),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        })
        .insert(Name::new("Ground"));
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
        .insert_resource(WindowDescriptor {
            width: WIDTH,
            height: HEIGHT,
            title: "bevy test".to_string(),
            resizable: false,
                    mode: Fullscreen,
            ..Default::default()
        })
        .add_startup_system(spawn_scene)
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_fruit)
        .add_plugins(DefaultPlugins)
        .add_plugins(DefaultPickingPlugins)
        .add_plugin(PlayerPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .run();
}
