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
        query::With,
        schedule::IntoSystemConfigs,
        system::{Commands, Query, Res, ResMut, Resource},
    },
    hierarchy::{BuildChildren, ChildBuild, ChildBuilder, Children, Parent},
    math::{Vec2, Vec3},
    sprite::Sprite,
    state::condition::in_state,
    transform::components::Transform,
    utils::default,
};
use once_cell::sync::Lazy;

use crate::{
    app_state::{AppState, Merge},
    clickable::Clickable,
    FollowMouse, Money, Size,
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
                Clickable::new()
                    .set_mousedown_event(FoxLotMousedownEvent)
                    .set_mouseup_event(FoxLotMouseupEvent),
                Size(*fox_lot_statics::SIZE),
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
        let mut fox_sanctuary = fox_lot.spawn((Self { level }, Transform::from_xyz(0., 0., 1.)));
        fox_sanctuary.insert(Sprite {
            image: asset_server.load(format!("images/fox{level}.png")),
            custom_size: Some(*fox_lot_statics::SIZE),
            ..default()
        });
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
struct FoxLotMousedownEvent(Entity);
#[derive(Event, Debug)]
struct FoxLotMouseupEvent(Entity);

pub struct FoxLotPlugin;
impl Plugin for FoxLotPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(FoxLotPrice::default())
            .add_event::<FoxLotMousedownEvent>()
            .add_event::<FoxLotMouseupEvent>()
            .add_systems(
                Update,
                (mousedown_fox_sanctuary, mouseup_fox_sanctuary).run_if(in_state(AppState::Merge)),
            );
    }
}
#[allow(clippy::needless_pass_by_value)]
fn mousedown_fox_sanctuary(
    mut commands: Commands,
    mut fox_lot_mousedown_events: EventReader<FoxLotMousedownEvent>,
    fox_lots_q: Query<&Children, With<FoxLot>>,
    mut fox_sanctuaries_q: Query<(Entity, Option<&Parent>, &FoxSanctuary, &Transform)>,
) {
    for ev in fox_lot_mousedown_events.read() {
        if let Ok(fox_lot_children) = fox_lots_q.get(ev.0) {
            for &fox_lot_child in fox_lot_children {
                if let Ok((entity, parent, fox_sanctuary, transform)) =
                    fox_sanctuaries_q.get_mut(fox_lot_child)
                {
                    if fox_sanctuary.level != 0 {
                        // Select Fox Sanctuary
                        commands
                            .entity(entity)
                            .insert(FollowMouse {
                                parent: parent.map(Parent::get),
                                previous_transform: *transform,
                            })
                            .remove_parent();
                    }
                }
            }
        }
    }
}
#[allow(clippy::needless_pass_by_value)]
fn mouseup_fox_sanctuary(
    asset_server: Res<AssetServer>,
    mut money: ResMut<Money>,
    mut fox_lot_price: ResMut<FoxLotPrice>,
    mut fox_lot_mouseup_events: EventReader<FoxLotMouseupEvent>,
    fox_lots_q: Query<&Children, With<FoxLot>>,
    mut fox_sanctuaries_q: Query<(&mut FoxSanctuary, &mut Sprite)>,
) {
    for ev in fox_lot_mouseup_events.read() {
        if money.ge(&fox_lot_price.0) {
            if let Ok(fox_lot_children) = fox_lots_q.get(ev.0) {
                for &fox_lot_child in fox_lot_children {
                    if let Ok((mut fox_sanctuary, mut fox_sanctuary_sprite)) =
                        fox_sanctuaries_q.get_mut(fox_lot_child)
                    {
                        if fox_sanctuary.level == 0 {
                            // Buy Fox Sanctuary
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
    }
}
