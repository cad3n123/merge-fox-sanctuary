#![allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]

use app_state::AppStatePlugin;
use bevy::{
    app::{App, Startup, Update},
    asset::AssetServer,
    core_pipeline::core_2d::Camera2d,
    ecs::{
        component::Component,
        entity::Entity,
        query::With,
        system::{Commands, Query, Res, Single},
    },
    math::{Vec2, Vec3},
    render::camera::Camera,
    transform::components::{GlobalTransform, Transform},
    window::{MonitorSelection, PrimaryWindow, Window, WindowMode},
    DefaultPlugins,
};
use clickable::{Clickable, ClickablePlugin};
use fox::FoxPlugin;
use merge::MergePlugin;
use money::Money;
use search::SearchPlugin;
use ui::UIPlugin;

macro_rules! impl_enum_distribution {
    ($t:ty) => {
        impl Distribution<$t> for StandardUniform {
            fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> $t {
                <$t>::from_repr(rng.random_range(0..<$t>::COUNT) as u32).unwrap()
            }
        }
    };
}

#[allow(dead_code)]
trait MyVec2 {
    fn into_vec3_with_z(self, z: f32) -> Vec3;
    fn into_vec3(self) -> Vec3;
}
impl MyVec2 for Vec2 {
    fn into_vec3_with_z(self, z: f32) -> Vec3 {
        Vec3 {
            x: self.x,
            y: self.y,
            z,
        }
    }
    fn into_vec3(self) -> Vec3 {
        self.into_vec3_with_z(0.)
    }
}

pub mod app_state;
pub mod clickable;
pub mod fox;
pub mod merge;
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
#[derive(Component, Debug)]
pub(crate) struct FollowMouse {
    parent: Option<Entity>,
    previous_transform: Transform,
}
impl FollowMouse {
    #[allow(clippy::needless_pass_by_value)]
    fn system(
        window: Single<&Window, With<PrimaryWindow>>,
        camera_q: Single<(&Camera, &GlobalTransform)>,
        mut follow_mouses_q: Query<&mut Transform, With<Self>>,
    ) {
        let (camera, camera_transform) = *camera_q;
        if let Some(mouse_coordinates) = mouse_world_coordinates(&window, camera, camera_transform)
        {
            for mut follow_mouse in &mut follow_mouses_q {
                let translation = &mut follow_mouse.translation;
                *translation = mouse_coordinates.into_vec3_with_z(translation.z);
            }
        }
    }
}
pub(crate) type Optional<'w, D, F = ()> = Option<Single<'w, D, F>>;

fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins,
        AppStatePlugin,
        UIPlugin,
        ClickablePlugin,
        FoxPlugin,
        MergePlugin,
        SearchPlugin,
    ));
    app.insert_resource(Money::default());
    app.add_systems(Startup, startup)
        .add_systems(Update, FollowMouse::system);
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

    merge::fox_lot::FoxLot::spawn_grid(&mut commands, &asset_server, -1..=1, -1..=1);
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
