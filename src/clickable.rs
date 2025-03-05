use std::{fmt::Debug, sync::Arc};

use bevy::{
    app::{App, Plugin, Update},
    ecs::{
        component::Component,
        entity::Entity,
        event::Event,
        query::With,
        schedule::IntoSystemConfigs,
        system::{Commands, Query},
        world::World,
    },
    input::{
        common_conditions::{input_just_pressed, input_just_released, input_pressed},
        mouse::MouseButton,
    },
    math::Vec2,
    render::camera::Camera,
    transform::components::{GlobalTransform, Transform},
    window::{PrimaryWindow, Window},
};

use crate::{mouse_world_coordinates, point_in_bounds, Size};

type OnClick = Arc<dyn Fn(&mut Commands, Entity) + Send + Sync>;
#[derive(Component, Clone)]
#[require(Size, Transform)]
pub(crate) struct Clickable {
    status: Option<Status>,
    on_click: OnClick,
    pub(crate) active: bool,
}
impl Clickable {
    pub fn new<F>(on_click: F) -> Self
    where
        F: Fn(&mut Commands, Entity) + Send + Sync + 'static,
    {
        Self {
            status: None,
            on_click: Arc::new(on_click),
            active: true,
        }
    }
    pub fn new_event<E>(event_constructor: fn(Entity) -> E) -> Self
    where
        E: Event + Send + Sync + Debug, // + 'static,
    {
        Self::new(move |commands: &mut Commands, clickable: Entity| {
            commands.queue(move |world: &mut World| {
                let event = event_constructor(clickable);
                world.send_event(event);
            });
        })
    }
}
#[derive(Clone, Copy, PartialEq, Eq)]
enum Status {
    Hovered,
    MouseDown,
}
pub struct ClickablePlugin;
impl Plugin for ClickablePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                (hover, mouse_down.run_if(input_pressed(MouseButton::Left))).chain(),
                mouse_up.run_if(input_just_released(MouseButton::Left)),
            ),
        );
    }
}
#[allow(clippy::needless_pass_by_value)]
fn hover(
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    mut clickables_q: Query<(&mut Clickable, &Size, &GlobalTransform)>,
) {
    let (camera, camera_transform) = q_camera.single();
    let window = q_window.single();

    if let Some(mouse_coordinates) = mouse_world_coordinates(window, camera, camera_transform) {
        for (mut clickable, size, transform) in &mut clickables_q {
            if !clickable.active {
                continue;
            }
            let translation = transform.translation();
            clickable.status = if point_in_bounds(
                mouse_coordinates,
                Vec2 {
                    x: translation.x - size.0.x / 2.,
                    y: translation.y - size.0.y / 2.,
                },
                size,
            ) {
                clickable.status.map_or(Some(Status::Hovered), Some)
            } else {
                None
            }
        }
    }
}
#[allow(clippy::needless_pass_by_value)]
fn mouse_down(mut clickables_q: Query<&mut Clickable>) {
    for mut clickable in &mut clickables_q {
        if !clickable.active {
            continue;
        }
        if clickable
            .status
            .is_some_and(|status| status == Status::Hovered)
        {
            clickable.status = Some(Status::MouseDown);
        }
    }
}
#[allow(clippy::needless_pass_by_value)]
fn mouse_up(mut commands: Commands, mut clickables_q: Query<(Entity, &mut Clickable)>) {
    for (entity, mut clickable) in &mut clickables_q {
        if !clickable.active {
            continue;
        }
        if clickable
            .status
            .is_some_and(|status| status == Status::MouseDown)
        {
            clickable.status = None;
            (clickable.on_click)(&mut commands, entity);
        }
    }
}
