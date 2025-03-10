use std::cmp::Ordering;

use animation::AnimationPlugin;
use bevy::{
    app::{App, Plugin},
    asset::AssetServer,
    ecs::{
        entity::Entity,
        system::{Commands, Query, Res, ResMut, Resource},
    },
    hierarchy::DespawnRecursiveExt,
    state::{
        app::AppExtStates,
        state::{NextState, OnExit, States},
    },
    window::SystemCursorIcon,
    winit::cursor::{CursorIcon, CustomCursor},
};
use cell::CellPlugin;
use ui::{CollectedFoxUI, UIPlugin};

use crate::{app_state::AppState, fox::Fox, merge::fox_lot::FoxSanctuary, Money};

pub mod animation;
pub mod cell;
pub mod ui;

#[derive(Resource, Default)]
pub(crate) struct Level(pub(crate) usize);
#[derive(Resource, Default, Debug, Clone, Copy)]
pub(crate) struct TotalFoxes(pub(crate) u32);
#[derive(Resource, Default, Debug, Clone, Copy)]
pub(crate) struct FoxesUncovered(u32);
#[derive(Resource)]
pub(crate) struct CatchPrice(Money);
impl Default for CatchPrice {
    fn default() -> Self {
        Self(Money::ZERO)
    }
}
#[derive(States, Default, Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SearchState {
    #[default]
    Reveal,
    Catch,
    Finished,
}
impl SearchState {
    fn set(
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        next_search_state: &mut ResMut<NextState<Self>>,
        window: Entity,
        new_state: Self,
    ) {
        next_search_state.set(new_state);
        let mut window_entity_commands = commands.entity(window);
        window_entity_commands.insert(match new_state {
            Self::Reveal | Self::Finished => CursorIcon::System(SystemCursorIcon::Default),
            Self::Catch => CursorIcon::Custom(CustomCursor::Image {
                handle: asset_server.load("images/fox-cursor.png"),
                hotspot: (20, 20),
            }),
        });
    }
}

pub(crate) struct SearchPlugin;
impl Plugin for SearchPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((UIPlugin, AnimationPlugin, CellPlugin))
            .insert_resource(Level::default())
            .insert_resource(TotalFoxes::default())
            .insert_resource(FoxesUncovered::default())
            .insert_resource(CatchPrice::default())
            .init_state::<SearchState>()
            .add_systems(OnExit(AppState::Search), exit);
    }
}
#[allow(clippy::needless_pass_by_value)]
pub(crate) fn exit(
    mut commands: Commands,
    collected_fox_uis_q: Query<(Entity, &CollectedFoxUI)>,
    mut fox_sanctuaries_q: Query<&mut FoxSanctuary>,
) {
    let mut foxes: Vec<Fox> = Vec::with_capacity(collected_fox_uis_q.iter().len());
    for (entity, collected_fox_ui) in &collected_fox_uis_q {
        commands.entity(entity).despawn_recursive();
        foxes.push(collected_fox_ui.0.clone());
    }
    while !foxes.is_empty() {
        let mut best_sanctuaries = vec![];
        for fox_sanctuary in &mut fox_sanctuaries_q {
            if !fox_sanctuary.has_room() {
                continue;
            }
            if best_sanctuaries.is_empty() {
                best_sanctuaries = vec![fox_sanctuary];
            } else {
                match fox_sanctuary.level().cmp(&best_sanctuaries[0].level()) {
                    Ordering::Greater => best_sanctuaries = vec![fox_sanctuary],
                    Ordering::Equal => best_sanctuaries.push(fox_sanctuary),
                    Ordering::Less => {}
                }
            }
        }
        assert_ne!(best_sanctuaries.len(), 0);
        let mut current_sanctuary = 0;
        while !foxes.is_empty() && current_sanctuary < best_sanctuaries.len() {
            if best_sanctuaries[current_sanctuary].has_room() {
                best_sanctuaries[current_sanctuary]
                    .foxes
                    .push(foxes.pop().unwrap());
            } else {
                current_sanctuary += 1;
            }
        }
    }
}
