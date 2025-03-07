use std::{fmt::Debug, sync::Arc};

use bevy::{
    app::{App, Plugin, Update},
    ecs::{
        component::Component,
        entity::Entity,
        event::Event,
        query::With,
        schedule::{IntoSystemConfigs, SystemSet},
        system::{Commands, Query, Single},
        world::World,
    },
    input::{
        common_conditions::{input_just_released, input_pressed},
        mouse::MouseButton,
    },
    math::Vec2,
    render::camera::Camera,
    transform::components::{GlobalTransform, Transform},
    window::{PrimaryWindow, Window},
};

use crate::{mouse_world_coordinates, point_in_bounds, Size};

#[derive(Component, Debug)]
pub(crate) struct Hovered;
#[derive(Component, Clone)]
#[require(Size, Transform)]
pub(crate) struct Clickable {
    status: Option<Status>,
    on_no_mouse_event: Option<MouseEvent>,
    on_hover: Option<MouseEvent>,
    on_mousedown: Option<MouseEvent>,
    on_mouseup: Option<MouseEvent>,
    pub(crate) active: bool,
}
impl Clickable {
    pub fn new() -> Self {
        Self {
            status: None,
            on_no_mouse_event: None,
            on_hover: None,
            on_mousedown: None,
            on_mouseup: None,
            active: true,
        }
    }
    fn new_mouse_event<E>(event_constructor: fn(Entity) -> E) -> MouseEvent
    where
        E: Event + Send + Sync + Debug,
    {
        Arc::new(move |commands: &mut Commands, clickable: Entity| {
            commands.queue(move |world: &mut World| {
                let event = event_constructor(clickable);
                world.send_event(event);
            });
        })
    }
    pub fn set_no_mouse_event_event<E>(mut self, event_constructor: fn(Entity) -> E) -> Self
    where
        E: Event + Send + Sync + Debug,
    {
        self.on_no_mouse_event = Some(Self::new_mouse_event(event_constructor));
        self
    }
    pub fn set_hover_event<E>(mut self, event_constructor: fn(Entity) -> E) -> Self
    where
        E: Event + Send + Sync + Debug,
    {
        self.on_hover = Some(Self::new_mouse_event(event_constructor));
        self
    }
    pub fn set_mousedown_event<E>(mut self, event_constructor: fn(Entity) -> E) -> Self
    where
        E: Event + Send + Sync + Debug,
    {
        self.on_mousedown = Some(Self::new_mouse_event(event_constructor));
        self
    }
    pub fn set_mouseup_event<E>(mut self, event_constructor: fn(Entity) -> E) -> Self
    where
        E: Event + Send + Sync + Debug,
    {
        self.on_mouseup = Some(Self::new_mouse_event(event_constructor));
        self
    }
}
#[derive(Clone, Copy, PartialEq, Eq)]
enum Status {
    Hovered,
    MouseDown,
}
type MouseEvent = Arc<dyn Fn(&mut Commands, Entity) + Send + Sync>;
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct ClickableSet;
pub struct ClickablePlugin;
impl Plugin for ClickablePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                (hover, mouse_down.run_if(input_pressed(MouseButton::Left)))
                    .in_set(ClickableSet)
                    .chain(),
                mouse_up
                    .run_if(input_just_released(MouseButton::Left))
                    .in_set(ClickableSet),
            ),
        );
    }
}
#[allow(clippy::needless_pass_by_value)]
fn hover(
    mut commands: Commands,
    window: Single<&Window, With<PrimaryWindow>>,
    q_camera: Single<(&Camera, &GlobalTransform)>,
    mut clickables_q: Query<(Entity, &mut Clickable, &Size, &GlobalTransform)>,
) {
    let (camera, camera_transform) = q_camera.into_inner();

    if let Some(mouse_coordinates) = mouse_world_coordinates(&window, camera, camera_transform) {
        for (entity, mut clickable, size, transform) in &mut clickables_q {
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
                clickable.status.map_or_else(
                    || {
                        if let Some(on_hover) = &clickable.on_hover {
                            (on_hover)(&mut commands, entity);
                        }
                        commands.entity(entity).insert(Hovered);
                        Some(Status::Hovered)
                    },
                    Some,
                )
            } else {
                if clickable.status.is_some() {
                    if let Some(on_no_mouse_event) = &clickable.on_no_mouse_event {
                        (on_no_mouse_event)(&mut commands, entity);
                    }
                    commands.entity(entity).remove::<Hovered>();
                }
                None
            }
        }
    }
}
#[allow(clippy::needless_pass_by_value)]
fn mouse_down(mut commands: Commands, mut clickables_q: Query<(Entity, &mut Clickable)>) {
    for (entity, mut clickable) in &mut clickables_q {
        if !clickable.active {
            continue;
        }
        if clickable
            .status
            .is_some_and(|status| status == Status::Hovered)
        {
            clickable.status = Some(Status::MouseDown);
            if let Some(on_mousedown) = &clickable.on_mousedown {
                (on_mousedown)(&mut commands, entity);
            }
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
            clickable.status = Some(Status::Hovered);
            if let Some(on_mouseup) = &clickable.on_mouseup {
                (on_mouseup)(&mut commands, entity);
            }
            if let Some(on_no_mouse_event) = &clickable.on_no_mouse_event {
                (on_no_mouse_event)(&mut commands, entity);
            }
        }
    }
}
