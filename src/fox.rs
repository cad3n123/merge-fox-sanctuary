use std::{rc::Rc, sync::Arc};

use enum_map::{enum_map, Enum, EnumMap};
use once_cell::sync::Lazy;
use rand::distr::{Distribution, StandardUniform};
use strum::EnumCount;
use strum_macros::{EnumCount, FromRepr};

use crate::IntVec2;

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
pub(crate) static FOX_SPECIES_LAYOUTS: Lazy<EnumMap<FoxSpecies, Vec<IntVec2>>> = Lazy::new(|| {
    enum_map! {
        FoxSpecies::Vulpes => vec![IntVec2 { x: 0, y: 1 }, IntVec2 { x: -1, y: 0 }, IntVec2 { x: 1, y: 0 }, IntVec2 { x: 0, y: -1 }], // TODO: Think of this
        FoxSpecies::Corsac => vec![IntVec2 { x: 1, y: -1 }, IntVec2 { x: 2, y: -1 }, IntVec2 { x: 1, y: -2 }, IntVec2 { x: 2, y: -2 }]
    }
});
impl_enum_distribution!(FoxSpecies);
#[derive(Clone)]
struct Name(Arc<str>);
static POSSIBLE_NAMES: Lazy<Vec<Name>> = Lazy::new(|| {
    vec![
        Name(Arc::from("Caden")),
        Name(Arc::from("Kylie")),
        Name(Arc::from("Red")),
        Name(Arc::from("Crash")),
        Name(Arc::from("Rusty")),
        Name(Arc::from("Ember")),
        Name(Arc::from("Fennec")),
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
        Name(Arc::from("Ochre")),
    ]
});
impl Distribution<Name> for StandardUniform {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Name {
        POSSIBLE_NAMES[rng.random_range(0..POSSIBLE_NAMES.len())].clone()
    }
}
struct Age(u32);
impl Age {
    const MAX_RANDOM_AGE: Self = Self(3);
}
impl Distribution<Age> for StandardUniform {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Age {
        Age(rng.random_range(0..Age::MAX_RANDOM_AGE.0))
    }
}
struct Fox {
    species: FoxSpecies,
    name: Name,
    age: Age,
    primary_injury: Injury,
    secondary_injury: Injury,
}
impl Fox {
    pub(crate) fn new_random() -> Self {
        let primary_injury: Injury = rand::random();
        Self {
            species: rand::random(),
            name: rand::random(),
            age: rand::random(),
            primary_injury,
            secondary_injury: {
                let mut secondary_injury: Injury = rand::random();
                while secondary_injury == primary_injury {
                    secondary_injury = rand::random();
                }
                secondary_injury
            },
        }
    }
}
#[derive(FromRepr, EnumCount, PartialEq, Eq, Clone, Copy)]
#[repr(u32)]
enum Injury {
    Malnourished,
    FracturedBone,
    Parasite,
    Disease,
    Trauma,
    Poisoned,
}
impl_enum_distribution!(Injury);
