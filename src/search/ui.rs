use bevy::{
    app::{App, Plugin, Update},
    asset::AssetServer,
    color::Color,
    ecs::{
        component::Component,
        entity::Entity,
        event::EventReader,
        query::{Changed, With},
        schedule::IntoSystemConfigs,
        system::{Commands, Query, Res, ResMut, Single},
    },
    hierarchy::{BuildChildren, ChildBuild, ChildBuilder},
    input::{common_conditions::input_just_released, keyboard::KeyCode},
    state::{
        condition::in_state,
        state::{NextState, OnEnter, State},
    },
    text::TextFont,
    ui::{
        widget::{Button, ImageNode, Text},
        AlignItems, AlignSelf, FlexDirection, FlexWrap, Interaction, JustifyContent, JustifySelf,
        Node, UiRect, Val,
    },
    utils::default,
    window::Window,
};

use crate::{
    app_state::{AppState, Search},
    fox::Fox,
    search::SearchState,
    ui::{CoinUI, MoneyContainer, RootTrait},
};

use super::{
    animation::{Fade, FadeMode},
    cell::{Cell, FoxCaughtEvent},
    CatchPrice,
};

#[derive(Component)]
struct Root;
impl RootTrait for Root {
    fn spawn(
        commands: &mut bevy::ecs::system::Commands,
        asset_server: &bevy::ecs::system::Res<bevy::asset::AssetServer>,
    ) {
        commands
            .spawn((
                Self,
                Search,
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
                CatchButton::spawn(root, asset_server);
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
struct CatchButton;
impl CatchButton {
    const FONT_SIZE: f32 = 50.;

    fn spawn(root: &mut ChildBuilder<'_>, asset_server: &Res<AssetServer>) {
        root.spawn((
            Self,
            Button,
            Node {
                align_self: AlignSelf::Center,
                align_items: AlignItems::Center,
                margin: UiRect::bottom(Val::Px(30.)),
                ..default()
            },
        ))
        .with_children(|search_button| {
            const COIN_SIZE: f32 = CatchButton::FONT_SIZE - 10.;
            const COIN_MARGIN: f32 = (CatchButton::FONT_SIZE - COIN_SIZE) * 0.5;
            CoinUI::spawn(
                search_button,
                asset_server,
                Val::Px(COIN_SIZE),
                Some(Val::Px(COIN_MARGIN)),
            );
            search_button.spawn((
                CatchPriceUI,
                Text::new("0"),
                TextFont::from_font_size(Self::FONT_SIZE),
            ));
            search_button.spawn((
                Text::new("Catch"),
                TextFont::from_font_size(Self::FONT_SIZE),
                Node {
                    margin: UiRect::left(Val::Px(Self::FONT_SIZE * 0.5)),
                    ..default()
                },
            ));
        });
    }
    #[allow(clippy::needless_pass_by_value)]
    fn system(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        search_state: Res<State<SearchState>>,
        mut next_search_state: ResMut<NextState<SearchState>>,
        window: Single<Entity, With<Window>>,
        button_interaction_q: Query<&Interaction, (Changed<Interaction>, With<Self>)>,
    ) {
        if button_interaction_q.is_empty() {
            return;
        }

        let search_button_interaction = button_interaction_q.single();
        if *search_button_interaction == Interaction::Pressed {
            SearchState::set(
                &mut commands,
                &asset_server,
                &mut next_search_state,
                *window,
                match search_state.get() {
                    SearchState::Reveal => SearchState::Catch,
                    SearchState::Catch => SearchState::Reveal,
                },
            );
        }
    }
}
#[derive(Component)]
struct CatchPriceUI;
impl CatchPriceUI {
    #[allow(clippy::needless_pass_by_value)]
    fn system(catch_price: Res<CatchPrice>, mut catch_price_uis_q: Query<&mut Text, With<Self>>) {
        for mut catch_price_ui in &mut catch_price_uis_q {
            *catch_price_ui = Text::from(catch_price.0.dollars_string());
        }
    }
}
#[derive(Component)]
struct FoxCollectionUI;
impl FoxCollectionUI {
    fn spawn(commands: &mut Commands) {
        commands.spawn((
            Self,
            Search,
            Button,
            Node {
                height: Val::Percent(100.),
                flex_direction: FlexDirection::Column,
                flex_wrap: FlexWrap::Wrap,
                ..default()
            },
        ));
    }
}
#[derive(Component)]
struct CollectedFoxUI(Fox);
impl CollectedFoxUI {
    const SIZE: f32 = Cell::SIZE;

    fn spawn(fox_collection_ui: &mut ChildBuilder<'_>, asset_server: &Res<AssetServer>, fox: Fox) {
        fox_collection_ui.spawn((
            Self(fox),
            Fade::new(FadeMode::Appearing),
            ImageNode {
                image: asset_server.load("images/Fox.png"),
                color: Color::srgba(1., 1., 1., 0.),
                ..default()
            },
            Node {
                width: Val::Px(Self::SIZE),
                height: Val::Px(Self::SIZE),
                ..default()
            },
        ));
    }
}

pub(super) struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Search), startup)
            .add_systems(
                Update,
                (
                    CatchButton::system,
                    CatchPriceUI::system,
                    set_search_state_reveal.run_if(input_just_released(KeyCode::Escape)),
                    on_fox_caught,
                )
                    .run_if(in_state(AppState::Search)),
            );
    }
}
#[allow(clippy::needless_pass_by_value)]
fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    Root::spawn(&mut commands, &asset_server);
    FoxCollectionUI::spawn(&mut commands);
}
#[allow(clippy::needless_pass_by_value)]
fn set_search_state_reveal(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut next_search_state: ResMut<NextState<SearchState>>,
    window: Single<Entity, With<Window>>,
) {
    SearchState::set(
        &mut commands,
        &asset_server,
        &mut next_search_state,
        *window,
        SearchState::Reveal,
    );
}
#[allow(clippy::needless_pass_by_value)]
fn on_fox_caught(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut fox_caught_event: EventReader<FoxCaughtEvent>,
    fox_collection_ui: Single<Entity, With<FoxCollectionUI>>,
) {
    for ev in fox_caught_event.read() {
        let fox_species = ev.0;
        let fox = Fox::new_random(fox_species);
        commands
            .entity(*fox_collection_ui)
            .with_children(|fox_collection_ui| {
                CollectedFoxUI::spawn(fox_collection_ui, &asset_server, fox);
            });
    }
}
