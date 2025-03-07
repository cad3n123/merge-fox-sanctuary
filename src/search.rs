use std::sync::Mutex;

use bevy::{
    app::{App, Plugin, Update},
    asset::{AssetServer, Assets},
    color::{
        palettes::tailwind::{GREEN_400, GREEN_800, GREEN_900},
        Color,
    },
    ecs::{
        component::Component,
        entity::Entity,
        event::{Event, EventReader},
        query::With,
        schedule::IntoSystemConfigs,
        system::{Commands, Query, Res, ResMut, Resource},
    },
    hierarchy::{BuildChildren, ChildBuild, ChildBuilder, Children, DespawnRecursiveExt, Parent},
    math::{primitives::Rectangle, Vec2, Vec3},
    render::{
        mesh::{Mesh, Mesh2d},
        view::Visibility,
    },
    sprite::{ColorMaterial, MeshMaterial2d, Sprite},
    state::{
        app::AppExtStates,
        condition::in_state,
        state::{OnEnter, State, States},
    },
    transform::components::Transform,
    utils::default,
};
use enum_map::Enum;
use once_cell::sync::Lazy;
use strum_macros::EnumString;

use crate::{
    app_state::{AppState, AppStateSet},
    clickable::Clickable,
    fox::FoxSpecies,
    Money, Size,
};

#[derive(Component, Clone, Copy, Default)]
struct Cell {
    cell_type: Option<CellType>,
    revealed: bool,
}
impl Cell {
    const SIZE: f32 = 100.;

    pub fn spawn(
        self,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        translation: Vec3,
    ) {
        let mut cell = commands.spawn((
            self,
            Mesh2d(meshes.add(Rectangle::from_length(Self::SIZE))),
            MeshMaterial2d(materials.add(Color::from(GREEN_400))),
            Transform::from_translation(translation),
        ));
        cell.with_children(|cell| {
            if !self.revealed {
                CellCover::spawn(cell, meshes, materials);
            }
            if let Some(cell_type) = &self.cell_type {
                cell_type.spawn(cell, asset_server, self.revealed);
            }
        });
    }
    pub fn spawn_level(
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        level: &Res<Level>,
    ) {
        let cells = &cell_statics::LEVEL_CELLS[level.0];
        let height = cells.len();
        let start_y = (height - 1) as f32 / 2.;
        for (y, row) in cells.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                cell.spawn(
                    commands,
                    asset_server,
                    meshes,
                    materials,
                    Vec3 {
                        x: x as f32 * Self::SIZE,
                        y: (start_y - y as f32) * Self::SIZE,
                        z: 0.,
                    },
                );
            }
        }
    }
}
impl From<ObstacleChar> for Cell {
    fn from(character: ObstacleChar) -> Self {
        Self {
            cell_type: match character.0 {
                // Fox Species
                'V' => Some(CellType::PawPrint(FoxSpecies::Vulpes)),
                'C' => Some(CellType::PawPrint(FoxSpecies::Corsac)),
                // Obstacles
                's' => Some(CellType::Obstacle(ObstacleType::Stones)),
                'l' => Some(CellType::Obstacle(ObstacleType::Log)),
                _ => None,
            },
            ..default()
        }
    }
}
impl From<FoxChar> for Cell {
    fn from(character: FoxChar) -> Self {
        Self {
            cell_type: match character.0 {
                'V' => Some(CellType::Fox(FoxSpecies::Vulpes)),
                'C' => Some(CellType::Fox(FoxSpecies::Corsac)),
                _ => None,
            },
            ..default()
        }
    }
}
struct ObstacleChar(char);
struct FoxChar(char);
#[derive(Component, Debug, EnumString, Clone, Copy)]
enum CellType {
    PawPrint(FoxSpecies),
    Obstacle(ObstacleType),
    Fox(FoxSpecies),
}
impl CellType {
    fn spawn(self, cell: &mut ChildBuilder<'_>, asset_server: &Res<AssetServer>, revealed: bool) {
        cell.spawn((
            self,
            Sprite {
                image: asset_server.load(format!(
                    "images/{}.png",
                    if self.is_fox() {
                        "Fox".to_owned() // TODO: Remove this if else and create different sprites for different foxes
                    } else {
                        format!("{self:?}")
                    }
                )),
                custom_size: Some(Vec2::splat(Cell::SIZE)),
                ..default()
            },
            Size(Vec2::splat(Cell::SIZE)),
            Transform::from_translation(Vec3::Z),
            if revealed {
                Visibility::Visible
            } else {
                Visibility::Hidden
            },
        ));
    }

    /// Returns `true` if the cell type is [`Fox`].
    ///
    /// [`Fox`]: CellType::Fox
    #[must_use]
    const fn is_fox(self) -> bool {
        matches!(self, Self::Fox(..))
    }
}
#[derive(Component)]
struct CellCover;
impl CellCover {
    const NORMAL_COLOR: Color = Color::Srgba(GREEN_800);
    const HOVER_COLOR: Color = Color::Srgba(GREEN_900);

    fn spawn(
        cell: &mut ChildBuilder<'_>,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) {
        cell.spawn((
            Self,
            Clickable::new()
                .set_no_mouse_event_event(CellCoverNoMouseEventEvent)
                .set_hover_event(CellCoverHoverEvent)
                .set_mouseup_event(CellCoverMouseupEvent),
            Size(Vec2::splat(Cell::SIZE)),
            Mesh2d(meshes.add(Rectangle::from_length(Cell::SIZE))),
            MeshMaterial2d(materials.add(Self::NORMAL_COLOR)),
            Transform::from_translation(2. * Vec3::Z),
        ));
    }
}
#[derive(Debug, Default, Clone, Copy, Enum)]
enum ObstacleType {
    #[default]
    Stones,
    Log,
}
#[derive(Resource, Default)]
struct Level(usize);
mod cell_statics {
    use std::cell;

    use once_cell::sync::Lazy;

    use super::{Cell, FoxChar, ObstacleChar};

    pub static LEVEL_CELLS: Lazy<Vec<Vec<Vec<Cell>>>> = Lazy::new(|| {
        vec![cells_from_level_layout(
            &vec![
                "C   ", //
                "l   ", //
                " s  ", //
                "    ", //
            ],
            &vec![
                "    ", //
                " C  ", //
                "    ", //
                "    ", //
            ],
        )]
    });
    fn cells_from_level_layout(obstacles: &Vec<&str>, foxes: &Vec<&str>) -> Vec<Vec<Cell>> {
        let mut cells: Vec<Vec<cell::Cell<(Cell, bool)>>> = vec![];
        for (obstacle_row, fox_row) in obstacles.iter().zip(foxes) {
            let mut cell_row = vec![];
            for (x, (obstacle_character, fox_character)) in
                obstacle_row.chars().zip(fox_row.chars()).enumerate()
            {
                let mut cell = if fox_character == ' ' {
                    Cell::from(ObstacleChar(obstacle_character))
                } else {
                    Cell::from(FoxChar(fox_character))
                };
                cell.revealed = x == 0;
                cell_row.push(cell::Cell::from((cell, true)));
            }
            cells.push(cell_row);
        }
        cells
            .iter()
            .map(|row| row.iter().map(|cell_cell| cell_cell.get().0).collect())
            .collect()
    }
}
#[derive(Event, Debug)]
struct CellCoverNoMouseEventEvent(Entity);
#[derive(Event, Debug)]
struct CellCoverHoverEvent(Entity);
#[derive(Event, Debug)]
struct CellCoverMouseupEvent(Entity);
#[derive(States, Default, Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SearchState {
    #[default]
    Reveal,
    Catch,
}
static CATCH_PRICE: Lazy<Mutex<Money>> = Lazy::new(|| Mutex::new(Money::new(0, 0)));

pub(crate) struct SearchPlugin;
impl Plugin for SearchPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Level::default())
            .init_state::<SearchState>()
            .add_event::<CellCoverNoMouseEventEvent>()
            .add_event::<CellCoverHoverEvent>()
            .add_event::<CellCoverMouseupEvent>()
            .add_systems(OnEnter(AppState::Search), search_startup.after(AppStateSet))
            .add_systems(
                Update,
                (no_mouse_event_cell, hover_cell, mouse_up_cell).run_if(in_state(AppState::Search)),
            );
    }
}

#[allow(clippy::needless_pass_by_value)]
fn search_startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    level: Res<Level>,
) {
    Cell::spawn_level(
        &mut commands,
        &asset_server,
        &mut meshes,
        &mut materials,
        &level,
    );
}
#[allow(clippy::needless_pass_by_value)]
fn no_mouse_event_cell(
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut cell_cover_event: EventReader<CellCoverNoMouseEventEvent>,
    mut cell_covers_q: Query<&mut MeshMaterial2d<ColorMaterial>, With<CellCover>>,
) {
    for ev in cell_cover_event.read() {
        if let Ok(mut cell_cover_material) = cell_covers_q.get_mut(ev.0) {
            *cell_cover_material = MeshMaterial2d(materials.add(CellCover::NORMAL_COLOR));
        }
    }
}
#[allow(clippy::needless_pass_by_value)]
fn hover_cell(
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut cell_cover_event: EventReader<CellCoverHoverEvent>,
    mut cell_covers_q: Query<&mut MeshMaterial2d<ColorMaterial>, With<CellCover>>,
) {
    for ev in cell_cover_event.read() {
        if let Ok(mut cell_cover_material) = cell_covers_q.get_mut(ev.0) {
            *cell_cover_material = MeshMaterial2d(materials.add(CellCover::HOVER_COLOR));
        }
    }
}
#[allow(clippy::needless_pass_by_value)]
fn mouse_up_cell(
    mut commands: Commands,
    search_state: Res<State<SearchState>>,
    mut money: ResMut<Money>,
    mut cell_cover_event: EventReader<CellCoverMouseupEvent>,
    cell_covers_q: Query<(&Parent, Entity), With<CellCover>>,
    mut cells_q: Query<(&mut Cell, &Children)>,
    mut cell_types_q: Query<&mut Visibility, With<CellType>>,
) {
    for ev in cell_cover_event.read() {
        if let Ok((cell_cover_parent, cell_cover)) = cell_covers_q.get(ev.0) {
            if let Ok((mut cell, cell_children)) = cells_q.get_mut(cell_cover_parent.get()) {
                cell.revealed = true;
                for cell_child in cell_children {
                    if let Ok(mut cell_type) = cell_types_q.get_mut(*cell_child) {
                        *cell_type = Visibility::Visible;
                    }
                }
            }
            commands.entity(cell_cover).despawn_recursive();
            if *search_state.get() == SearchState::Catch {
                *money -= CATCH_PRICE.lock().unwrap().clone();
            }
        }
    }
}
