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
    let mut triangle_path = kurbo::BezPath::new();
    triangle_path.move_to(kurbo::Point::new(0.0, -10.0));
    triangle_path.line_to(kurbo::Point::new(0.0, 10.0));
    triangle_path.line_to(kurbo::Point::new(20.0, 0.0));
    triangle_path.close_path();

    let triangle = VelloBezPath::new().with_path(triangle_path);
    let head_style = (
        HeadFill(Fill::new().with_color(Color::WHITE.with_alpha(0.6))),
        HeadStroke(
            Stroke::from_style(kurbo::Stroke::new(4.0).with_join(kurbo::Join::Miter))
                .with_color(Color::BLACK.with_alpha(0.6)),
        ),
    );

    // Line
    let line = (
        VelloLine::new(DVec2::new(100.0, 100.0), DVec2::new(0.0, -100.0)),
        Stroke::new(5.0).with_color(Color::WHITE),
        Transform::from_xyz(-300.0, 0.0, 0.0),
        HeadBundle::new(triangle.clone()),
        head_style.clone(),
    );

    // Rectangle
    let rect = (
        VelloRect::new(100.0, 200.0),
        Fill::new().with_color(css::ORANGE.into()),
        Stroke::new(5.0).with_color(css::RED.into()),
        Transform::from_xyz(-100.0, 0.0, 0.0),
        HeadBundle::new(triangle.clone()),
        head_style.clone(),
    );

    // Circle
    let circle = (
        VelloCircle::new(50.0),
        Fill::new().with_color(css::YELLOW_GREEN.into()),
        Stroke::new(5.0).with_color(css::DARK_GREEN.into()),
        Transform::from_xyz(100.0, 0.0, 0.0),
        HeadBundle::new(triangle.clone()),
        head_style.clone(),
    );

    let mut bez_path = kurbo::BezPath::new();
    bez_path.move_to((0.0, 100.0));
    bez_path.curve_to((-100.0, 50.0), (100.0, -50.0), (0.0, -100.0));
    bez_path.close_path();

    // Bézier Path
    let bezier_path = (
        VelloBezPath::new().with_path(bez_path),
        Stroke::new(4.0).with_color(css::YELLOW.into()),
        Transform::from_xyz(300.0, 0.0, 0.0),
        HeadBundle::new(triangle),
        head_style,
    );

    commands.spawn(VelloSceneBundle::default()).insert(line);
    commands.spawn(VelloSceneBundle::default()).insert(rect);
    commands.spawn(VelloSceneBundle::default()).insert(circle);
    commands
        .spawn(VelloSceneBundle::default())
        .insert(bezier_path);
}

fn animation(mut q_heads: Query<&mut Head>, time: Res<Time>) {
    // Overshoots for stability check
    let mut factor = time.elapsed_seconds_f64() * 0.5;
    factor = factor.sin().remap(-1.0, 1.0, -0.2, 1.2);

    for mut head in q_heads.iter_mut() {
        head.time = factor;
        // head.rotation_offset = std::f64::consts::TAU * factor;
    }
}
