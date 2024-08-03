use bevy::{color::palettes::css, math::DVec2, prelude::*};
use bevy_vello_graphics::{bevy_vello::prelude::*, prelude::*};

fn main() {
    App::new()
        // Bevy plugins
        .add_plugins(DefaultPlugins)
        // Custom Plugins
        .add_plugins(VelloGraphicsPlugin)
        .add_systems(Startup, (setup, render_shapes))
        .add_systems(Update, animation)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn render_shapes(mut commands: Commands) {
    // Line
    let line = (
        VelloLine::new(DVec2::new(100.0, 100.0), DVec2::new(0.0, -100.0)),
        Fill::new().with_color(Color::WHITE),
        Stroke::new(5.0).with_color(Color::WHITE),
        Transform::from_xyz(-300.0, 0.0, 0.0),
        HeadBundle::new(VelloRect::new(20.0, 20.0)),
    );

    // Rectangle
    let rect = (
        VelloRect::new(100.0, 200.0),
        Fill::new().with_color(css::ORANGE.into()),
        Stroke::new(5.0).with_color(css::RED.into()),
        Transform::from_xyz(-100.0, 0.0, 0.0),
        HeadBundle::new(VelloRect::new(20.0, 20.0)),
    );

    // Circle
    let circle = (
        VelloCircle::new(50.0),
        Fill::new().with_color(css::YELLOW_GREEN.into()),
        Stroke::new(5.0).with_color(css::DARK_GREEN.into()),
        Transform::from_xyz(100.0, 0.0, 0.0),
        HeadBundle::new(VelloRect::new(20.0, 20.0)),
    );

    let mut bez_path = kurbo::BezPath::new();
    bez_path.move_to((300.0, 100.0));
    bez_path.curve_to((200.0, 50.0), (400.0, -50.0), (300.0, -100.0));

    // Bézier Path
    let bezier_path = (
        VelloBezPath::new().with_path(bez_path),
        Stroke::new(4.0).with_color(css::YELLOW.into()),
        HeadBundle::new(VelloRect::new(20.0, 20.0)),
    );

    commands.spawn(VelloSceneBundle::default()).insert(line);
    commands.spawn(VelloSceneBundle::default()).insert(rect);
    commands.spawn(VelloSceneBundle::default()).insert(circle);
    commands
        .spawn(VelloSceneBundle::default())
        .insert(bezier_path);
}

fn animation(mut q_heads: Query<&mut Head>, time: Res<Time>) {
    let factor = time.elapsed_seconds_f64().sin() * 0.5 + 0.5;

    for mut head in q_heads.iter_mut() {
        head.time = factor;
        // head.rotation_offset = std::f64::consts::TAU * factor;
    }
}
