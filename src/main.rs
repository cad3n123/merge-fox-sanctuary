use app_state::AppStatePlugin;
use bevy::{
    app::{App, Startup},
    asset::AssetServer,
    core_pipeline::core_2d::Camera2d,
    ecs::{
        component::Component, query::With, system::{Commands, Query, Res}
    },
    math::Vec2,
    render::camera::Camera,
    transform::components::GlobalTransform,
    window::{MonitorSelection, PrimaryWindow, Window, WindowMode},
    DefaultPlugins,
};
use clickable::{Clickable, ClickablePlugin};
use fox_lot::{FoxLot, FoxLotPlugin};
use money::Money;
use search::SearchPlugin;
use ui::{merge_ui, search_ui, UIPlugin};

pub mod app_state;
pub mod clickable;
pub mod fox_lot;
pub mod money;
pub mod search;
pub mod ui;

#[derive(Component)]
struct Size(Vec2);
impl Default for Size {
    fn default() -> Self {
        Self(Vec2::ZERO)
    }
}

fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins,
        AppStatePlugin,
        UIPlugin,
        merge_ui::UIPlugin,
        search_ui::UIPlugin,
        ClickablePlugin,
        FoxLotPlugin,
        SearchPlugin,
    ));
    app.insert_resource(Money::default());
    app.add_systems(Startup, startup);
    app.run();
}
#[allow(clippy::needless_pass_by_value)]
fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut windows_q: Query<&mut Window, With<PrimaryWindow>>,
) {
    let mut window = windows_q.single_mut();
    window.mode = WindowMode::BorderlessFullscreen(MonitorSelection::Current);

    commands.spawn(Camera2d);

    FoxLot::spawn_grid(&mut commands, &asset_server, -1..=1, -1..=1);
}
fn mouse_world_coordinates(
    window: &Window,
    camera: &Camera,
    camera_transform: &GlobalTransform,
) -> Option<Vec2> {
    window.cursor_position().and_then(|cursor_pos| {
        camera
            .viewport_to_world_2d(camera_transform, cursor_pos)
            .ok()
    })
}
fn point_in_bounds(point: Vec2, top_left: Vec2, size: &Size) -> bool {
    point.x >= top_left.x
        && point.x <= top_left.x + size.0.x
        && point.y >= top_left.y
        && point.y <= top_left.y + size.0.y
}
