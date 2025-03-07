use bevy::{
    app::{App, Plugin, Update},
    asset::AssetServer,
    ecs::{
        component::Component,
        query::With,
        schedule::{common_conditions::resource_changed, Condition, IntoSystemConfigs},
        system::{Commands, Query, Res},
    },
    hierarchy::{BuildChildren, ChildBuild, ChildBuilder},
    state::condition::state_changed,
    text::TextFont,
    ui::{
        widget::{ImageNode, Text},
        AlignItems, Node, Val,
    },
    utils::default,
};

use crate::{app_state::AppState, Money};

pub(crate) trait RootTrait {
    fn spawn(commands: &mut Commands, asset_server: &Res<AssetServer>);
}
#[derive(Component)]
pub(crate) struct MoneyContainer;
impl MoneyContainer {
    const FONT_SIZE: f32 = 85.;

    pub(crate) fn spawn(top_container: &mut ChildBuilder<'_>, asset_server: &Res<AssetServer>) {
        top_container
            .spawn((
                Self,
                Node {
                    column_gap: Val::Px(10.),
                    align_items: AlignItems::Center,
                    ..default()
                },
            ))
            .with_children(|money_container| {
                money_container.spawn((
                    ImageNode::new(asset_server.load("images/coin.png")),
                    Node {
                        width: Val::Px(Self::FONT_SIZE),
                        height: Val::Px(Self::FONT_SIZE),
                        ..default()
                    },
                ));
                money_container.spawn((
                    MoneyUI,
                    Text::new("0"),
                    TextFont::from_font_size(Self::FONT_SIZE),
                ));
            });
    }
}
#[derive(Component)]
struct MoneyUI;
impl MoneyUI {
    #[allow(clippy::needless_pass_by_value)]
    fn update(mut money_uis_q: Query<&mut Text, With<Self>>, money: Res<Money>) {
        let money_string: String = money.to_string();
        for mut money_ui in &mut money_uis_q {
            money_ui.0.clone_from(&money_string);
        }
    }
}
pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            MoneyUI::update.run_if(resource_changed::<Money>.or(state_changed::<AppState>)),
        );
    }
}
