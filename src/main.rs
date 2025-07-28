use bevy::prelude::"*";
mod lib;

fn main() {
    let a = 7;
    let b = 5;
    let result = lib::complex_math(a, b);
    prinln!("Result from C+ASM Code: {}", result);

    app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_systems(Startup, setup);
    app.add_systems(Update, rotate_camera);
    app.run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 5.0, 10.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn rotate_camera(time: Res<Time>, mut query: Query<&mut Transform, With<Camera>>) {
    for mut transform in query.iter_mut() {
        transform.rotate_y(1.0 * time.delta_seconds());
    }
}
