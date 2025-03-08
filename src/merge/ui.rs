use bevy::{
    app::{App, Plugin, Startup, Update},
    asset::AssetServer,
    ecs::{
        component::Component,
        query::{Changed, With},
        schedule::{common_conditions::resource_changed, IntoSystemConfigs},
        system::{Commands, Query, Res, ResMut},
    },
    hierarchy::{BuildChildren, ChildBuild, ChildBuilder},
    state::{
        condition::in_state,
        state::NextState,
    },
    text::TextFont,
    ui::{
        widget::{Button, Text},
        AlignItems, AlignSelf, FlexDirection, Interaction, JustifyContent, JustifySelf, Node,
        UiRect, Val,
    },
    utils::default,
};

use crate::{
    app_state::{AppState, Merge},
    ui::{CoinUI, MoneyContainer, RootTrait},
};

use super::fox_lot::FoxLotPrice;

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
            PriceContainer::spawn(top_container, asset_server);
        });
    }
}
#[derive(Component)]
struct PriceContainer;
impl PriceContainer {
    const FONT_SIZE: f32 = 35.;

    fn spawn(top_container: &mut ChildBuilder<'_>, asset_server: &Res<AssetServer>) {
        top_container
            .spawn((
                Self,
                Node {
                    column_gap: Val::Px(10.),
                    align_items: AlignItems::Center,
                    ..default()
                },
            ))
            .with_children(|price_container| {
                price_container.spawn((Text::new("Price: "), TextFont::from_font_size(45.)));
                CoinUI::spawn(
                    price_container,
                    asset_server,
                    Val::Px(Self::FONT_SIZE),
                    None,
                );
                price_container.spawn((
                    FoxLotPriceUI,
                    Text::new("0"),
                    TextFont::from_font_size(Self::FONT_SIZE),
                ));
            });
    }
}
#[derive(Component)]
struct FoxLotPriceUI;
impl FoxLotPriceUI {
    #[allow(clippy::needless_pass_by_value)]
    fn update(
        mut fox_lot_price_ui_q: Query<&mut Text, With<Self>>,
        fox_lot_price: Res<FoxLotPrice>,
    ) {
        let mut fox_lot_price_ui = fox_lot_price_ui_q.single_mut();
        fox_lot_price_ui.0 = fox_lot_price.to_string();
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
        search_button_interaction_q: Query<&Interaction, (Changed<Interaction>, With<Self>)>,
    ) {
        if search_button_interaction_q.is_empty() {
            return;
        }

        let search_button_interaction = search_button_interaction_q.single();
        if *search_button_interaction == Interaction::Pressed {
            next_app_state.set(AppState::Search);
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
            (
                FoxLotPriceUI::update.run_if(resource_changed::<FoxLotPrice>),
                SearchButton::system,
            )
                .run_if(in_state(AppState::Merge)),
        );
    }
}
#[allow(clippy::needless_pass_by_value)]
pub(super) fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    Root::spawn(&mut commands, &asset_server);
}
