use std::{fmt::Display, sync::Arc};

use bevy::{
    color::{palettes::tailwind::ORANGE_400, Color, Srgba},
    ecs::component::Component,
    hierarchy::{ChildBuild, ChildBuilder},
    math::{Vec2, Vec3},
    sprite::Sprite,
    transform::components::Transform,
};
use enum_map::Enum;
use once_cell::sync::Lazy;
use rand::distr::{Distribution, StandardUniform};
use strum::EnumCount;
use strum_macros::{EnumCount, FromRepr};

use crate::{money::Cent, Money};

macro_rules! impl_enum_distribution {
    ($t:ty) => {
        impl Distribution<$t> for StandardUniform {
            fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> $t {
                <$t>::from_repr(rng.random_range(0..<$t>::COUNT) as u32).unwrap()
            }
        }
    };
}

#[derive(FromRepr, EnumCount, Debug, Default, Clone, Copy, Enum)]
#[repr(u32)]
pub(crate) enum FoxSpecies {
    #[default]
    Vulpes,
    Corsac,
}
impl_enum_distribution!(FoxSpecies);
impl Display for FoxSpecies {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
#[derive(Debug, Clone)]
pub(crate) struct Name(Arc<str>);
impl Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
static POSSIBLE_NAMES: Lazy<Vec<Name>> = Lazy::new(|| {
    vec![
        Name(Arc::from("Caden")),
        Name(Arc::from("Kylie")),
        Name(Arc::from("Red")),
        Name(Arc::from("Crash")),
        Name(Arc::from("Bonzo")),
        Name(Arc::from("Rusty")),
        Name(Arc::from("Ember")),
        Name(Arc::from("Blaze")),
        Name(Arc::from("Sienna")),
        Name(Arc::from("Autumn")),
        Name(Arc::from("Copper")),
        Name(Arc::from("Maple")),
        Name(Arc::from("Amber")),
        Name(Arc::from("Cinnamon")),
        Name(Arc::from("Marigold")),
        Name(Arc::from("Pumpkin")),
        Name(Arc::from("Flare")),
        Name(Arc::from("Ash")),
        Name(Arc::from("Crimson")),
        Name(Arc::from("Scarlet")),
        Name(Arc::from("Tangerine")),
        Name(Arc::from("Mango")),
        Name(Arc::from("Sunny")),
        Name(Arc::from("Sorrel")),
        Name(Arc::from("Dusk")),
        Name(Arc::from("Ginger")),
        Name(Arc::from("Poppy")),
        Name(Arc::from("Hazel")),
        Name(Arc::from("Mochi")),
        Name(Arc::from("Toffee")),
        Name(Arc::from("Chai")),
        Name(Arc::from("Basil")),
        Name(Arc::from("Nutmeg")),
        Name(Arc::from("Yuki")),
        Name(Arc::from("Akira")),
        Name(Arc::from("Haru")),
        Name(Arc::from("Kitsu")),
        Name(Arc::from("Kyo")),
        Name(Arc::from("Renji")),
        Name(Arc::from("Tora")),
        Name(Arc::from("Sable")),
        Name(Arc::from("Fawn")),
        Name(Arc::from("Willow")),
        Name(Arc::from("Nova")),
        Name(Arc::from("Vixen")),
        Name(Arc::from("Freya")),
        Name(Arc::from("Echo")),
        Name(Arc::from("Luna")),
        Name(Arc::from("Celeste")),
        Name(Arc::from("Comet")),
        Name(Arc::from("Orion")),
        Name(Arc::from("Zorro")),
        Name(Arc::from("Bandit")),
        Name(Arc::from("Shadow")),
        Name(Arc::from("Phantom")),
        Name(Arc::from("Whisper")),
        Name(Arc::from("Mirage")),
        Name(Arc::from("Drift")),
        Name(Arc::from("Zephyr")),
        Name(Arc::from("Flicker")),
        Name(Arc::from("Glint")),
        Name(Arc::from("Spark")),
        Name(Arc::from("Wisp")),
        Name(Arc::from("Dandelion")),
        Name(Arc::from("Pebble")),
        Name(Arc::from("Clover")),
        Name(Arc::from("Thistle")),
        Name(Arc::from("Ivy")),
        Name(Arc::from("Fern")),
        Name(Arc::from("Birch")),
        Name(Arc::from("Mistral")),
        Name(Arc::from("Solstice")),
        Name(Arc::from("Frost")),
        Name(Arc::from("Boreal")),
        Name(Arc::from("Aurora")),
        Name(Arc::from("Aria")),
        Name(Arc::from("Eclipse")),
        Name(Arc::from("Quasar")),
        Name(Arc::from("Nebula")),
        Name(Arc::from("Meteor")),
        Name(Arc::from("Aether")),
        Name(Arc::from("Rune")),
        Name(Arc::from("Cipher")),
        Name(Arc::from("Jinx")),
        Name(Arc::from("Myst")),
        Name(Arc::from("Vortex")),
        Name(Arc::from("Lyric")),
        Name(Arc::from("Sonnet")),
        Name(Arc::from("Rhapsody")),
        Name(Arc::from("Melody")),
        Name(Arc::from("Cadence")),
        Name(Arc::from("Harper")),
        Name(Arc::from("Briar")),
        Name(Arc::from("Saffron")),
        Name(Arc::from("Velvet")),
        Name(Arc::from("Garnet")),
        Name(Arc::from("Topaz")),
        Name(Arc::from("Amberly")),
        Name(Arc::from("Tawny")),
        Name(Arc::from("Okra")),
    ]
});
impl Distribution<Name> for StandardUniform {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Name {
        POSSIBLE_NAMES[rng.random_range(0..POSSIBLE_NAMES.len())].clone()
    }
}
#[derive(Debug, Clone)]
pub(crate) struct Age(u32);
impl Age {
    const MAX_RANDOM_AGE: Self = Self(6);
}
impl Display for Age {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} year{} old",
            self.0,
            if self.0 == 1 { "" } else { "s" }
        )
    }
}
impl Distribution<Age> for StandardUniform {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Age {
        Age(rng.random_range(0..Age::MAX_RANDOM_AGE.0))
    }
}
#[derive(Component, Debug, Clone)]
pub(crate) struct Fox {
    species: FoxSpecies,
    name: Name,
    age: Age,
    favorite_activity: Activity,
    primary_problem: Problem,
    secondary_problem: Problem,
}
static FOX_COLOR: Lazy<Color> = Lazy::new(|| Color::from(Fox::SRGBA));
impl Fox {
    const WIDTH: f32 = 10.;
    const HEIGHT: f32 = Self::WIDTH / 2.;
    const SRGBA: Srgba = ORANGE_400;

    pub(crate) fn spawn(&self, fox_sanctuary: &mut ChildBuilder<'_>, translation: Vec3) {
        fox_sanctuary.spawn((
            self.clone(),
            Transform::from_translation(translation),
            Sprite::from_color(*FOX_COLOR, Vec2::new(Self::WIDTH, Self::HEIGHT)),
        ));
    }

    pub(crate) fn new_random(species: FoxSpecies) -> Self {
        let primary_problem = Problem::new(rand::random());
        Self {
            species,
            name: rand::random(),
            age: rand::random(),
            favorite_activity: Activity::new(),
            primary_problem: primary_problem.clone(),
            secondary_problem: {
                let mut secondary_problem = Problem::new(rand::random());
                while secondary_problem.problem_type == primary_problem.problem_type {
                    secondary_problem.problem_type = rand::random();
                }
                secondary_problem
            },
        }
    }

    pub(crate) const fn name(&self) -> &Name {
        &self.name
    }

    pub(crate) const fn age(&self) -> &Age {
        &self.age
    }

    pub(crate) const fn favorite_activity_type(&self) -> ActivityType {
        self.favorite_activity.activity_type
    }

    pub(crate) const fn primary_problem_type(&self) -> ProblemType {
        self.primary_problem.problem_type
    }

    pub(crate) const fn species(&self) -> FoxSpecies {
        self.species
    }

    pub(crate) fn income(&self) -> Money {
        Money::from(Cent(
            10 + if self.favorite_activity.satisfied {
                10
            } else {
                0
            } + if self.primary_problem.fixed { 10 } else { 0 }
                + if self.secondary_problem.fixed { 10 } else { 0 },
        ))
    }
}
#[derive(Debug, Clone)]
struct Activity {
    activity_type: ActivityType,
    satisfied: bool,
}
impl Activity {
    fn new() -> Self {
        Self {
            activity_type: rand::random(),
            satisfied: false,
        }
    }
}
#[derive(Debug, FromRepr, EnumCount, PartialEq, Eq, Clone, Copy)]
#[repr(u32)]
pub(crate) enum ActivityType {
    Pouncing,
    Digging,
    Playing,
    Hunting,
    Tunneling,
    Exploring,
    Sunbathing,
}
impl Display for ActivityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Pouncing => "Pouncing",
                Self::Digging => "Digging",
                Self::Playing => "Playing",
                Self::Hunting => "Hunting",
                Self::Tunneling => "Tunneling",
                Self::Exploring => "Exploring",
                Self::Sunbathing => "Sunbathing",
            }
        )
    }
}
#[derive(Debug, Clone)]
struct Problem {
    #[allow(clippy::struct_field_names)]
    problem_type: ProblemType,
    known: bool,
    fixed: bool,
}
impl Problem {
    const fn new(problem_type: ProblemType) -> Self {
        Self {
            problem_type,
            known: false,
            fixed: false,
        }
    }
}
impl_enum_distribution!(ActivityType);
#[derive(Debug, FromRepr, EnumCount, PartialEq, Eq, Clone, Copy)]
#[repr(u32)]
pub(crate) enum ProblemType {
    Malnourished,
    FracturedBone,
    Parasite,
    Disease,
    Trauma,
    Poisoned,
}
impl Display for ProblemType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Malnourished => "Malnourished",
                Self::FracturedBone => "Fractured Bone",
                Self::Parasite => "Parasite",
                Self::Disease => "Disease",
                Self::Trauma => "Trauma",
                Self::Poisoned => "Poisoned",
            }
        )
    }
}
impl_enum_distribution!(ProblemType);
