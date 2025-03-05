use bevy::{
    app::{App, Startup},
    asset::AssetServer,
    core_pipeline::core_2d::Camera2d,
    ecs::{
        component::Component,
        entity::Entity,
        query::With,
        system::{Commands, Query, Res},
    },
    math::Vec2,
    render::{camera::Camera, view::Visibility},
    state::{
        app::AppExtStates,
        state::{OnEnter, OnExit, States},
    },
    transform::components::GlobalTransform,
    window::{MonitorSelection, PrimaryWindow, Window, WindowMode},
    DefaultPlugins,
};
use clickable::{Clickable, ClickablePlugin};
use fox_lot::{FoxLot, FoxLotPlugin};
use money::Money;
use search::SearchPlugin;
use ui::merge_ui;

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
#[derive(Component)]
struct Merge;
#[derive(States, Default, Debug, Hash, Clone, PartialEq, Eq)]
enum AppState {
    #[default]
    Merge,
    Search,
}

fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins,
        merge_ui::UIPlugin,
        ClickablePlugin,
        FoxLotPlugin,
        SearchPlugin,
    ));
    app.insert_resource(Money::default());
    app.init_state::<AppState>();
    app.add_systems(Startup, startup)
        .add_systems(OnEnter(AppState::Merge), merge_startup)
        .add_systems(OnExit(AppState::Merge), merge_exit);
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
#[allow(clippy::needless_pass_by_value)]
fn merge_startup(
    mut commands: Commands,
    mut merge_entities_q: Query<(Entity, Option<&mut Clickable>), With<Merge>>,
) {
    for (merge_entity, clickable) in &mut merge_entities_q {
        commands.entity(merge_entity).insert(Visibility::Visible);
        if let Some(mut clickable) = clickable {
            clickable.active = true;
        }
    }
}
#[allow(clippy::needless_pass_by_value)]
fn merge_exit(
    mut commands: Commands,
    mut merge_entities_q: Query<(Entity, Option<&mut Clickable>), With<Merge>>,
) {
    for (merge_entity, clickable) in &mut merge_entities_q {
        commands.entity(merge_entity).insert(Visibility::Hidden);
        if let Some(mut clickable) = clickable {
            clickable.active = false;
        }
    }
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
