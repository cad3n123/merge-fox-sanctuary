use std::{
    fmt::Display,
    ops::{RangeInclusive, SubAssign},
};

use bevy::{
    app::{App, Plugin, Update},
    asset::AssetServer,
    ecs::{
        component::Component,
        entity::Entity,
        event::{Event, EventReader},
        query::{With, Without},
        schedule::IntoSystemConfigs,
        system::{Commands, Query, Res, ResMut, Resource, Single},
    },
    hierarchy::{BuildChildren, ChildBuild, ChildBuilder, Parent},
    math::{Vec2, Vec3},
    sprite::Sprite,
    state::condition::in_state,
    transform::{commands::BuildChildrenTransformExt, components::Transform},
    utils::default,
};
use once_cell::sync::Lazy;

use crate::{
    app_state::{AppState, Merge},
    clickable::{Clickable, ClickableSet, Hovered},
    FollowMouse, Money, Optional, Size,
};

#[derive(Component)]
pub(crate) struct FoxLot;
mod fox_lot_statics {
    use super::{FoxLot, Lazy, Vec2};

    pub static SIZE: Lazy<Vec2> = Lazy::new(|| Vec2::new(100., 100.));
    pub static PADDED_SIZE: Lazy<Vec2> = Lazy::new(|| *SIZE + FoxLot::PADDING);
}
impl FoxLot {
    const PADDING: f32 = 10.;

    fn spawn(
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        translation: Vec3,
        level: i32,
    ) {
        commands
            .spawn((
                Self,
                Merge,
                Transform::from_translation(translation),
                Sprite {
                    image: asset_server.load("images/fox-lot.png"),
                    custom_size: Some(*fox_lot_statics::SIZE),
                    ..default()
                },
            ))
            .with_children(|fox_lot| {
                FoxSanctuary::spawn(fox_lot, asset_server, level);
            });
    }
    fn spawn_at_grid_pos(
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        x: i32,
        y: i32,
        level: i32,
    ) {
        Self::spawn(
            commands,
            asset_server,
            Vec3 {
                x: x as f32 * fox_lot_statics::PADDED_SIZE.x,
                y: y as f32 * fox_lot_statics::PADDED_SIZE.y,
                z: 0.,
            },
            level,
        );
    }
    pub(crate) fn spawn_grid(
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        y_s: RangeInclusive<i32>,
        x_s: RangeInclusive<i32>,
    ) {
        for y in y_s {
            for x in x_s.clone() {
                Self::spawn_at_grid_pos(commands, asset_server, x, y, 0);
            }
        }
    }
}
#[derive(Component)]
struct FoxSanctuary {
    #[allow(dead_code)]
    level: i32,
}
impl FoxSanctuary {
    fn spawn(fox_lot: &mut ChildBuilder<'_>, asset_server: &Res<AssetServer>, level: i32) {
        fox_lot.spawn((
            Self { level },
            Transform::from_xyz(0., 0., 1.),
            Sprite {
                image: asset_server.load(format!("images/fox{level}.png")),
                custom_size: Some(*fox_lot_statics::SIZE),
                ..default()
            },
            Clickable::new()
                .set_mousedown_event(FoxSanctuaryMousedownEvent)
                .set_mouseup_event(FoxSanctuaryMouseupEvent),
            Size(*fox_lot_statics::SIZE),
        ));
    }
}
#[derive(Resource)]
pub(crate) struct FoxLotPrice(Money);
mod fox_lot_price_statics {
    use crate::Money;

    use super::Lazy;

    pub static BASE_PRICE: Lazy<Money> = Lazy::new(|| Money::new(50, 0));
}
impl Default for FoxLotPrice {
    fn default() -> Self {
        Self(fox_lot_price_statics::BASE_PRICE.clone())
    }
}
impl Display for FoxLotPrice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Event, Debug)]
struct FoxSanctuaryMousedownEvent(Entity);
#[derive(Event, Debug)]
struct FoxSanctuaryMouseupEvent(Entity);

pub struct FoxLotPlugin;
impl Plugin for FoxLotPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(FoxLotPrice::default())
            .add_event::<FoxSanctuaryMousedownEvent>()
            .add_event::<FoxSanctuaryMouseupEvent>()
            .add_systems(
                Update,
                select_fox_sanctuary
                    .before(ClickableSet)
                    .run_if(in_state(AppState::Merge)),
            )
            .add_systems(
                Update,
                (mousedown_fox_sanctuary, buy_fox_sanctuary)
                    .after(ClickableSet)
                    .run_if(in_state(AppState::Merge)),
            );
    }
}
#[allow(clippy::needless_pass_by_value)]
fn mousedown_fox_sanctuary(
    mut commands: Commands,
    mut fox_lot_mousedown_events: EventReader<FoxSanctuaryMousedownEvent>,
    mut fox_sanctuaries_q: Query<(Entity, &Parent, &FoxSanctuary, &Transform)>,
) {
    for ev in fox_lot_mousedown_events.read() {
        if let Ok((entity, parent, fox_sanctuary, transform)) = fox_sanctuaries_q.get_mut(ev.0) {
            if fox_sanctuary.level != 0 {
                // Select Fox Sanctuary
                commands
                    .entity(entity)
                    .insert(FollowMouse {
                        // parent: parent.map(Parent::get),
                        parent: Some(parent.get()),
                        previous_transform: *transform,
                    })
                    .remove_parent_in_place();
            }
            // commands.entity(entity).remove::<Hovered>();
        }
    }
}
#[allow(clippy::needless_pass_by_value)]
fn buy_fox_sanctuary(
    asset_server: Res<AssetServer>,
    mut money: ResMut<Money>,
    mut fox_lot_price: ResMut<FoxLotPrice>,
    mut fox_sanctuary_mouseup_events: EventReader<FoxSanctuaryMouseupEvent>,
    mut fox_sanctuaries_q: Query<(&mut FoxSanctuary, &mut Sprite)>,
) {
    for ev in fox_sanctuary_mouseup_events.read() {
        if money.ge(&fox_lot_price.0) {
            if let Ok((mut fox_sanctuary, mut fox_sanctuary_sprite)) =
                fox_sanctuaries_q.get_mut(ev.0)
            {
                // Buy Fox Sanctuary
                if fox_sanctuary.level == 0 {
                    money.sub_assign(fox_lot_price.0.clone());
                    fox_lot_price.0 += &*fox_lot_price_statics::BASE_PRICE;
                    fox_sanctuary.level += 1;
                    fox_sanctuary_sprite.image =
                        asset_server.load(format!("images/fox{}.png", fox_sanctuary.level));
                }
            }
        }
    }
}
type HoveredTypeSanctuaryData<'a> = (Entity, &'a Parent, &'a mut Transform);
type HoveredTypeSanctuaryFilter = (With<FoxSanctuary>, With<Hovered>, Without<FollowMouse>);
//type HoveredFoxSanctuary =
#[allow(clippy::needless_pass_by_value)]
fn select_fox_sanctuary(
    mut commands: Commands,
    mut fox_sanctuary_mouseup_events: EventReader<FoxSanctuaryMouseupEvent>,
    mut fox_sanctuaries_q: Query<(Entity, &mut Transform, &FollowMouse)>,
    hovered_fox_sanctuary_q: Optional<HoveredTypeSanctuaryData, HoveredTypeSanctuaryFilter>,
) {
    let mut hovered_fox_sanctuary_q = hovered_fox_sanctuary_q.map(Single::into_inner);

    for ev in fox_sanctuary_mouseup_events.read() {
        if let Ok((entity, mut fox_sanctuary_transform, follow_mouse)) =
            fox_sanctuaries_q.get_mut(ev.0)
        {
            // Move Fox Sanctuary
            let follow_parent = follow_mouse.parent.unwrap();
            if let Some((hovered_entity, hovered_parent, ref mut hovered_transform)) =
                hovered_fox_sanctuary_q
            {
                commands.entity(entity).set_parent(hovered_parent.get());
                commands.entity(hovered_entity).set_parent(follow_parent);
                hovered_transform.translation.x = 0.;
                hovered_transform.translation.y = 0.;
            } else {
                commands.entity(entity).set_parent(follow_parent);
            }
            commands.entity(entity).remove::<FollowMouse>();
            commands.entity(entity).remove::<Hovered>();
            fox_sanctuary_transform.translation.x = 0.;
            fox_sanctuary_transform.translation.y = 0.;
        }
    }
}
