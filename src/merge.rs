use bevy::app::{App, Plugin};
use fox_lot::FoxLotPlugin;
use ui::UIPlugin;

pub mod fox_lot;
pub mod ui;

pub(crate) struct MergePlugin;
impl Plugin for MergePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((UIPlugin, FoxLotPlugin));
    }
}
