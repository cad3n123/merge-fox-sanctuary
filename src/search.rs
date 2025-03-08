use bevy::{
    app::{App, Plugin},
    asset::AssetServer,
    ecs::{
        entity::Entity,
        system::{Commands, Res, ResMut, Resource},
    },
    state::{
        app::AppExtStates,
        state::{NextState, States},
    },
    window::SystemCursorIcon,
    winit::cursor::{CursorIcon, CustomCursor},
};
use cell::CellPlugin;
use ui::UIPlugin;

use crate::Money;

pub mod cell;
pub mod ui;

#[derive(Resource, Default)]
pub(crate) struct Level(usize);
#[derive(Resource, Default, Debug, Clone, Copy)]
pub(crate) struct TotalFoxes(u32);
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
            Self::Reveal => CursorIcon::System(SystemCursorIcon::Default),
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
        app.add_plugins((UIPlugin, CellPlugin))
            .insert_resource(Level::default())
            .insert_resource(TotalFoxes::default())
            .insert_resource(CatchPrice::default())
            .init_state::<SearchState>();
    }
}
