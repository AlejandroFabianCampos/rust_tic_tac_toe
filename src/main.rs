use bevy::{prelude::*, sprite::*};

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.init_gizmo_group::<CustomGizmos>()
            .add_systems(Startup, setup)
            .add_systems(
                Update,
                (draw_gizmos, check_mouse_clicks, update_tile_colors),
            );
    }
}

fn main() {
    App::new().add_plugins((DefaultPlugins, AppPlugin)).run();
}

#[derive(Default, Reflect, GizmoConfigGroup)]
struct CustomGizmos {}

const NO_COLOR: Color = Color::Rgba {
    red: 0.0,
    green: 0.0,
    blue: 0.0,
    alpha: 0.0,
};

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    for i in 0..3 {
        for j in 0..3 {
            let position_x = ((i as f32) * 200.) - 200.;
            let position_y = ((j as f32) * 200.) - 200.;
            let position = Vec2::new(position_x, position_y);
            println!("position_x: {} - position_y: {}", position_x, position_y);
            let mesh_entity = commands
                .spawn(MaterialMesh2dBundle {
                    mesh: meshes.add(Rectangle::default()).into(),
                    material: materials.add(NO_COLOR),
                    transform: Transform::from_xyz(position.x, position.y, 0.0).with_scale(Vec3 {
                        x: 200.,
                        y: 200.,
                        z: 0.,
                    }),
                    ..default()
                })
                .id();
            commands.spawn(GameTile {
                position: Vec2::new(position_x, position_y),
                size: Vec2::new(200., 200.),
                number: (i * 3 + j) + 1,
                mesh_bundle_entity: mesh_entity,
            });
        }
    }
}

fn draw_gizmos(mut gizmos: Gizmos) {
    let delimitator_color = Color::WHITE;
    // vertical delimitator 1
    gizmos.line_2d(
        Vec2::new(-100., -300.),
        Vec2::new(-100., 300.),
        delimitator_color,
    );
    // vertical delimitator 2
    gizmos.line_2d(
        Vec2::new(100., -300.),
        Vec2::new(100., 300.),
        delimitator_color,
    );
    // horizontal delimitator 1
    gizmos.line_2d(
        Vec2::new(-300., -100.),
        Vec2::new(300., -100.),
        delimitator_color,
    );
    // horizontal delimitator 2
    gizmos.line_2d(
        Vec2::new(-300., 100.),
        Vec2::new(300., 100.),
        delimitator_color,
    );

    gizmos.rect_2d(Vec2::ZERO, 0.0, Vec2::splat(600.), Color::WHITE);
}

#[derive(Component)]
struct GameTile {
    position: Vec2,
    size: Vec2,
    number: i8,
    mesh_bundle_entity: Entity,
}

impl GameTile {
    fn contains(&self, point: Vec2) -> bool {
        (self.position.x - self.size.x * 0.5..self.position.x + self.size.x * 0.5)
            .contains(&point.x)
            && (self.position.y - self.size.y * 0.5..self.position.y + self.size.y * 0.5)
                .contains(&point.y)
    }

    fn contains_cursor(&self, window: &Window) -> bool {
        if let Some(position) = window.cursor_position() {
            let relative_position = window_absolute_to_relative(window.clone(), position);

            if self.contains(relative_position) {
                // The click is within this tile
                println!(
                    "tile {} at {:?} does contain the click",
                    self.number, self.position
                );
                return true;
            }
        }
        return false;
    }
}

fn window_absolute_to_relative(window: Window, absolute_position: Vec2) -> Vec2 {
    let mut relative_position: Vec2 = absolute_position;

    relative_position.x -= window.width() / 2.0;
    relative_position.y = window.height() / 2.0 - relative_position.y;

    return relative_position;
}

fn window_relative_to_absolute(window: Window, relative_position: Vec2) -> Vec2 {
    let mut absolute_position: Vec2 = relative_position;

    absolute_position.x += window.width() / 2.0;
    absolute_position.y = window.height() / 2.0 - absolute_position.y;

    return absolute_position;
}

fn update_tile_colors(
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut mesh_query: Query<&Handle<ColorMaterial>>,
    windows: Query<&Window>,
    tile_query: Query<&GameTile>,
) {
    let window = windows.single();
    for game_tile in tile_query.iter() {
        if game_tile.contains_cursor(window) {
            if let Ok(material_handle) = mesh_query.get_mut(game_tile.mesh_bundle_entity) {
                if let Some(material) = materials.get_mut(material_handle) {
                    material.color = Color::BLUE; // Change color to blue
                }
            }
        } else {
            if let Ok(material_handle) = mesh_query.get_mut(game_tile.mesh_bundle_entity) {
                if let Some(material) = materials.get_mut(material_handle) {
                    material.color = NO_COLOR; // Change color to blue
                }
            }
        }
    }
}

fn check_mouse_clicks(
    buttons: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    query: Query<&GameTile>,
) {
    let window = windows.single();
    if buttons.just_pressed(MouseButton::Left) {
        if Some(window.cursor_position()).is_some() {
            for game_tile in query.iter() {
                if game_tile.contains_cursor(window) {
                    // The click is within this tile
                    println!(
                        "Clicked on tile {} at {:?}",
                        game_tile.number, game_tile.position
                    );
                }
            }
        }
    }
}
