use bevy::{
    app::{App, Plugin}, ecs::{component::Component, entity::Entity, query::With, schedule::{IntoSystemConfigs, SystemSet}, system::{Commands, Query}}, render::view::Visibility, state::{app::AppExtStates, state::{OnEnter, OnExit, States}}
};

use crate::Clickable;

#[derive(States, Default, Debug, Hash, Clone, PartialEq, Eq)]
pub(crate) enum AppState {
    #[default]
    Merge,
    Search,
}
#[derive(Component)]
pub(crate) struct Merge;
#[derive(Component)]
pub(crate) struct Search;
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct AppStateSet;

pub struct AppStatePlugin;
impl Plugin for AppStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>();
        app.add_systems(OnEnter(AppState::Merge), merge_startup.in_set(AppStateSet))
            .add_systems(OnExit(AppState::Merge), merge_exit.in_set(AppStateSet));
    }
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
pub(crate) fn merge_exit(
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
