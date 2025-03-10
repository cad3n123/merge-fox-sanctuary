use std::time::Duration;

use bevy::{
    app::{Plugin, Update},
    color::Alpha,
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

trait Fadable {
    const STRENGTH: f32 = 2.;

    fn set_alpha(&mut self, lucency: u32);
    fn get_alpha(lucency: u32) -> f32 {
        (lucency as f32 / Fade::MAX_LUCENCY as f32).powi(2)
    }
}
impl Fadable for Sprite {
    fn set_alpha(&mut self, lucency: u32) {
        self.color.set_alpha(Self::get_alpha(lucency));
    }
}
impl Fadable for ImageNode {
    fn set_alpha(&mut self, lucency: u32) {
        self.color.set_alpha(Self::get_alpha(lucency));
    }
}
#[derive(Component)]
pub(crate) struct Fade {
    pub(crate) mode: FadeMode,
    pub(crate) speed: FadeSpeed,
    pub(crate) end_mode: Option<FadeEndMode>,
    lucency: u32,
}
impl Fade {
    const MAX_LUCENCY: u32 = 50;

    pub(crate) const fn new(
        mode: FadeMode,
        speed: FadeSpeed,
        end_mode: Option<FadeEndMode>,
    ) -> Self {
        Self {
            mode,
            speed,
            end_mode,
            lucency: mode.default_lucency(),
        }
    }
    #[allow(clippy::needless_pass_by_value)]
    fn system<T: Fadable + Component>(
        mut commands: Commands,
        time: Res<Time>,
        mut fade_timer: ResMut<FadeTimer>,
        mut fades_q: Query<(Entity, &mut Self, &mut T)>,
    ) {
        fade_timer.timer.tick(time.delta());

        if fade_timer.timer.finished() {
            for (entity, mut fade, mut fade_component) in &mut fades_q {
                fade.lucency = match &fade.mode {
                    FadeMode::Appearing => fade.lucency + fade.speed as u32,
                    FadeMode::Disappearing => fade.lucency.saturating_sub(fade.speed as u32),
                };
                fade_component.set_alpha(fade.lucency);
                if fade.mode == FadeMode::Appearing && fade.lucency >= Self::MAX_LUCENCY
                    || fade.mode == FadeMode::Disappearing && fade.lucency == 0
                {
                    let mut entity_commands = commands.entity(entity);
                    if let Some(ref end_mode) = fade.end_mode {
                        match end_mode {
                            FadeEndMode::Delete => {
                                entity_commands.remove::<Self>();
                                entity_commands.despawn_recursive();
                            }
                            FadeEndMode::BounceRepeat => {
                                fade.mode.toggle();
                            }
                            FadeEndMode::BounceOnce(new_fade_end_mode) => {
                                fade.end_mode = *new_fade_end_mode.clone();
                                fade.mode.toggle();
                            }
                        }
                    } else {
                        entity_commands.remove::<Self>();
                    }
                }
            }
        }
    }
}
#[derive(Clone, Copy)]
pub(crate) enum FadeSpeed {
    Slow = 1,
    Medium = 2,
    Fast = 4,
}
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum FadeMode {
    Appearing,
    Disappearing,
}
impl FadeMode {
    const fn default_lucency(self) -> u32 {
        match self {
            Self::Appearing => 0,
            Self::Disappearing => Fade::MAX_LUCENCY,
        }
    }
    fn toggle(&mut self) {
        *self = match self {
            Self::Appearing => Self::Disappearing,
            Self::Disappearing => Self::Appearing,
        }
    }
}
#[derive(Clone, PartialEq, Eq)]
pub(crate) enum FadeEndMode {
    Delete,
    BounceRepeat,
    BounceOnce(Box<Option<FadeEndMode>>),
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
            (Fade::system::<Sprite>, Fade::system::<ImageNode>).run_if(in_state(AppState::Search)),
        );
    }
}
