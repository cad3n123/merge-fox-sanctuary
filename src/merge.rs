use bevy::app::{App, Plugin};
use ui::UIPlugin;

pub mod ui;

pub(crate) struct MergePlugin;
impl Plugin for MergePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(UIPlugin);
    }
}
