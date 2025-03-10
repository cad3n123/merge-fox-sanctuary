use bevy::{
    app::{App, Plugin, Startup},
    ecs::{
        schedule::IntoSystemConfigs,
        system::{Query, ResMut, Resource},
    },
    state::state::OnEnter,
};
use fox_lot::{FoxLotPlugin, FoxSanctuary};
use ui::UIPlugin;

use crate::{app_state::AppState, search, Money};

pub mod fox_lot;
pub mod ui;

#[derive(Resource, Debug)]
struct Income(Money);
impl Default for Income {
    fn default() -> Self {
        Self(Money::ZERO)
    }
}

pub(crate) struct MergePlugin;
impl Plugin for MergePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Income::default())
            .add_plugins((UIPlugin, FoxLotPlugin))
            .add_systems(Startup, calculate_income)
            .add_systems(
                OnEnter(AppState::Merge),
                calculate_income.after(search::exit),
            );
    }
}
#[allow(clippy::needless_pass_by_value)]
fn calculate_income(mut income: ResMut<Income>, fox_sanctuaries_q: Query<&FoxSanctuary>) {
    income.0 = Money::ZERO;
    for fox_sanctuary in &fox_sanctuaries_q {
        for fox in &fox_sanctuary.foxes {
            income.0 += fox.income();
        }
    }
    println!("New income: {income:?}");
}
