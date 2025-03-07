use bevy::{
    app::{App, Plugin, Update},
    asset::AssetServer,
    ecs::{
        component::Component,
        entity::Entity,
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
        widget::{Button, Text},
        AlignItems, AlignSelf, FlexDirection, Interaction, JustifyContent, JustifySelf, Node,
        UiRect, Val,
    },
    utils::default,
    window::Window,
    winit::cursor::{CursorIcon, CustomCursor},
};

use crate::{
    app_state::{AppState, Search},
    search::SearchState,
    ui::{MoneyContainer, RootTrait},
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
                CatchButton::spawn(root);
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
            CatchButtonText::spawn(search_button);
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
struct CatchButtonText;
impl CatchButtonText {
    const FONT_SIZE: f32 = 50.;

    fn spawn(search_button: &mut ChildBuilder<'_>) {
        search_button.spawn((
            Self,
            Text::new("Catch"),
            TextFont::from_font_size(Self::FONT_SIZE),
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
                    set_search_state_reveal.run_if(input_just_released(KeyCode::Escape)),
                )
                    .run_if(in_state(AppState::Search)),
            );
    }
}
#[allow(clippy::needless_pass_by_value)]
fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    Root::spawn(&mut commands, &asset_server);
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
