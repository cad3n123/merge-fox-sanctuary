use std::time::Duration;

use bevy::{
    app::{Plugin, Update},
    color::Color,
    ecs::{
        component::Component,
        entity::Entity,
        schedule::IntoSystemConfigs,
        system::{Commands, Query, Res, ResMut, Resource},
    },
    hierarchy::DespawnRecursiveExt,
    sprite::Sprite,
    state::condition::in_state,
    time::{Time, Timer, TimerMode},
    ui::widget::ImageNode,
};

use crate::app_state::AppState;

#[derive(Component)]
pub(crate) struct Fade {
    mode: FadeMode,
    lucency: u32,
}
impl Fade {
    const MAX_LUCENCY: u32 = 30;

    pub(crate) const fn new(mode: FadeMode) -> Self {
        Self {
            lucency: mode.default_lucency(),
            mode,
        }
    }
    #[allow(clippy::needless_pass_by_value)]
    fn image_node_system(
        mut commands: Commands,
        time: Res<Time>,
        mut fade_timer: ResMut<FadeTimer>,
        mut fades_q: Query<(Entity, &mut Self, &mut ImageNode)>,
    ) {
        fade_timer.timer.tick(time.delta());

        if fade_timer.timer.finished() {
            for (entity, mut fade, mut fade_sprite) in &mut fades_q {
                fade.lucency = match &fade.mode {
                    FadeMode::Appearing => fade.lucency + 1,
                    FadeMode::Disappearing(_) => fade.lucency - 1,
                };
                fade_sprite.color =
                    Color::srgba(1., 1., 1., fade.lucency as f32 / Self::MAX_LUCENCY as f32);
                if fade.mode == FadeMode::Appearing && fade.lucency >= Self::MAX_LUCENCY
                    || fade.mode.discriminant()
                        == FadeMode::Disappearing(DisappearingMode::Delete).discriminant()
                        && fade.lucency == 0
                {
                    let mut entity_commands = commands.entity(entity);
                    entity_commands.remove::<Self>();
                    if fade.mode == FadeMode::Disappearing(DisappearingMode::Delete) {
                        entity_commands.despawn_recursive();
                        println!("Despawning fade outer!");
                    }
                }
            }
        }
    }
    #[allow(clippy::needless_pass_by_value)]
    fn sprite_system(
        mut commands: Commands,
        time: Res<Time>,
        mut fade_timer: ResMut<FadeTimer>,
        mut fades_q: Query<(Entity, &mut Self, &mut Sprite)>,
    ) {
        fade_timer.timer.tick(time.delta());

        if fade_timer.timer.finished() {
            for (entity, mut fade, mut fade_sprite) in &mut fades_q {
                fade.lucency = match &fade.mode {
                    FadeMode::Appearing => fade.lucency + 1,
                    FadeMode::Disappearing(_) => fade.lucency - 1,
                };
                fade_sprite.color =
                    Color::srgba(1., 1., 1., fade.lucency as f32 / Self::MAX_LUCENCY as f32);
                if fade.mode == FadeMode::Appearing && fade.lucency >= Self::MAX_LUCENCY
                    || fade.mode.discriminant()
                        == FadeMode::Disappearing(DisappearingMode::Delete).discriminant()
                        && fade.lucency == 0
                {
                    let mut entity_commands = commands.entity(entity);
                    entity_commands.remove::<Self>();
                    if fade.mode == FadeMode::Disappearing(DisappearingMode::Delete) {
                        entity_commands.despawn_recursive();
                    }
                }
            }
        }
    }
}
#[derive(PartialEq, Eq)]
#[repr(u8)]
pub(crate) enum FadeMode {
    Appearing,
    Disappearing(DisappearingMode),
}
impl FadeMode {
    const fn default_lucency(&self) -> u32 {
        match self {
            Self::Appearing => 0,
            Self::Disappearing(_) => Fade::MAX_LUCENCY,
        }
    }
    fn discriminant(&self) -> u8 {
        // SAFETY: Because `Self` is marked `repr(u8)`, its layout is a `repr(C)` `union`
        // between `repr(C)` structs, each of which has the `u8` discriminant as its first
        // field, so we can read the discriminant without offsetting the pointer.
        unsafe { *<*const _>::from(self).cast::<u8>() }
    }
}
#[derive(PartialEq, Eq)]
pub(crate) enum DisappearingMode {
    Remain,
    Delete,
}
#[derive(Resource)]
struct FadeTimer {
    timer: Timer,
}
impl FadeTimer {
    const DURATION_MILLIS: u64 = 30;
}

pub(super) struct AnimationPlugin;
impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.insert_resource(FadeTimer {
            timer: Timer::new(
                Duration::from_millis(FadeTimer::DURATION_MILLIS),
                TimerMode::Repeating,
            ),
        })
        .add_systems(
            Update,
            (Fade::image_node_system, Fade::sprite_system).run_if(in_state(AppState::Search)),
        );
    }
}
