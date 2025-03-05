use bevy::{
    app::{App, Plugin},
    ecs::{
        component::Component,
        entity::Entity,
        query::With,
        schedule::{IntoSystemConfigs, SystemSet},
        system::{Commands, Query},
    },
    render::view::Visibility,
    state::{
        app::AppExtStates,
        state::{OnEnter, OnExit, States},
    },
};

use crate::Clickable;

#[derive(States, Default, Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub(crate) enum AppState {
    #[default]
    Merge,
    Search,
}

macro_rules! add_state_transitions {
    ($self:ident, $( $variant:ident ),* ) => {
        $self
        $(
            .add_state_transition_systems::<$variant>(AppState::$variant)
        )*
    };
}

#[derive(Component)]
pub(crate) struct Merge;
#[derive(Component)]
pub(crate) struct Search;
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct AppStateSet;

trait StateTransitionExtension {
    fn add_state_transition_systems<T: Component>(&mut self, state: AppState) -> &mut Self;
    fn configure_state_transitions(&mut self) -> &mut Self;
}

impl StateTransitionExtension for App {
    fn add_state_transition_systems<T: Component>(&mut self, state: AppState) -> &mut Self {
        self.add_systems(OnEnter(state), app_state_enter::<T>.in_set(AppStateSet))
            .add_systems(OnExit(state), app_state_exit::<T>.in_set(AppStateSet))
    }

    fn configure_state_transitions(&mut self) -> &mut Self {
        add_state_transitions!(self, Merge, Search)
    }
}

pub struct AppStatePlugin;
impl Plugin for AppStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>();
        app.configure_state_transitions();
    }
}
#[allow(clippy::needless_pass_by_value)]
fn app_state_enter<T: Component>(
    mut commands: Commands,
    mut entities_q: Query<(Entity, Option<&mut Clickable>), With<T>>,
) {
    for (entity, clickable) in &mut entities_q {
        commands.entity(entity).insert(Visibility::Visible);
        if let Some(mut clickable) = clickable {
            clickable.active = true;
        }
    }
}
#[allow(clippy::needless_pass_by_value)]
pub(crate) fn app_state_exit<T: Component>(
    mut commands: Commands,
    mut entities_q: Query<(Entity, Option<&mut Clickable>), With<T>>,
) {
    for (entity, clickable) in &mut entities_q {
        commands.entity(entity).insert(Visibility::Hidden);
        if let Some(mut clickable) = clickable {
            clickable.active = false;
        }
    }
}
