use bevy::{
    app::{App, Plugin, Startup, Update},
    asset::AssetServer,
    color::{palettes::tailwind::GRAY_300, Color},
    ecs::{
        component::Component,
        query::{Changed, With},
        schedule::{common_conditions::resource_changed, IntoSystemConfigs},
        system::{Commands, Query, Res, ResMut},
    },
    hierarchy::{BuildChildren, ChildBuild, ChildBuilder},
    math::{Vec2, Vec3},
    sprite::Sprite,
    state::{condition::in_state, state::NextState},
    text::{JustifyText, Text2d, TextFont, TextLayout},
    transform::components::Transform,
    ui::{
        widget::{Button, Text},
        AlignItems, AlignSelf, BackgroundColor, FlexDirection, Interaction, JustifyContent,
        JustifySelf, Node, UiRect, Val,
    },
    utils::default,
};

use crate::{
    app_state::{AppState, Merge},
    search::{cell::LEVEL_CELLS, Level},
    ui::{CoinUI, MoneyContainer, RootTrait},
};

use super::{fox_lot::FoxLotPrice, FoxStorageInfo};

#[derive(Component)]
struct Root;
impl RootTrait for Root {
    fn spawn(commands: &mut Commands, asset_server: &Res<AssetServer>) {
        commands
            .spawn((
                Self,
                Merge,
                Node {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::SpaceBetween,
                    ..default()
                },
            ))
            .with_children(|root| {
                TopContainer::spawn(root, asset_server);
                SearchButton::spawn(root);
            });
    }
}
#[derive(Component)]
struct TopContainer;
impl TopContainer {
    fn spawn(root: &mut ChildBuilder<'_>, asset_server: &Res<AssetServer>) {
        root.spawn((
            Self,
            Node {
                width: Val::Percent(100.),
                row_gap: Val::Px(10.),
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                justify_self: JustifySelf::Start,
                ..default()
            },
        ))
        .with_children(|top_container| {
            MoneyContainer::spawn(top_container, asset_server);
        });
    }
}
#[derive(Component)]
struct SearchButton;
impl SearchButton {
    fn spawn(root: &mut ChildBuilder<'_>) {
        root.spawn((
            Self,
            Button,
            Node {
                align_self: AlignSelf::Center,
                margin: UiRect::bottom(Val::Px(30.)),
                ..default()
            },
        ))
        .with_children(|search_button| {
            SearchButtonText::spawn(search_button);
        });
    }
    #[allow(clippy::needless_pass_by_value)]
    fn system(
        mut next_app_state: ResMut<NextState<AppState>>,
        level: Res<Level>,
        fox_storage_info: Res<FoxStorageInfo>,
        search_button_interaction_q: Query<&Interaction, (Changed<Interaction>, With<Self>)>,
    ) {
        if search_button_interaction_q.is_empty() {
            return;
        }

        let search_button_interaction = search_button_interaction_q.single();
        if *search_button_interaction == Interaction::Pressed {
            // Check fox sanctuary capacity
            let total_foxes = &LEVEL_CELLS[level.0].1;
            if fox_storage_info.remaining_capacity() >= total_foxes.0 {
                next_app_state.set(AppState::Search);
            }
        }
    }
}
#[derive(Component)]
struct SearchButtonText;
impl SearchButtonText {
    const FONT_SIZE: f32 = 50.;

    fn spawn(search_button: &mut ChildBuilder<'_>) {
        search_button.spawn((
            Self,
            Text::new("Search"),
            TextFont::from_font_size(Self::FONT_SIZE),
        ));
    }
}

pub(super) struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup).add_systems(
            Update,
            SearchButton::system.run_if(in_state(AppState::Merge)),
        );
    }
}
#[allow(clippy::needless_pass_by_value)]
pub(super) fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    Root::spawn(&mut commands, &asset_server);
}
