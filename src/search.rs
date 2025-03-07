use std::sync::Mutex;

use bevy::{
    app::{App, Plugin},
    ecs::system::Resource,
    state::{app::AppExtStates, state::States},
};
use cell::CellPlugin;
use once_cell::sync::Lazy;
use ui::UIPlugin;

use crate::Money;

pub mod cell;
pub mod ui;

#[derive(Resource, Default)]
pub(crate) struct Level(usize);
#[derive(States, Default, Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SearchState {
    #[default]
    Reveal,
    Catch,
}
static CATCH_PRICE: Lazy<Mutex<Money>> = Lazy::new(|| Mutex::new(Money::new(0, 0)));

pub(crate) struct SearchPlugin;
impl Plugin for SearchPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((UIPlugin, CellPlugin))
            .insert_resource(Level::default())
            .init_state::<SearchState>();
    }
}
