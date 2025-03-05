use bevy::{
    app::{App, Plugin, Update},
    asset::{AssetServer, Assets},
    color::{
        palettes::tailwind::{GREEN_400, GREEN_800},
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
    hierarchy::{BuildChildren, ChildBuild, ChildBuilder, DespawnRecursiveExt, Parent},
    math::{primitives::Rectangle, Vec2, Vec3},
    render::mesh::{Mesh, Mesh2d},
    sprite::{ColorMaterial, MeshMaterial2d, Sprite},
    state::{condition::in_state, state::OnEnter},
    transform::components::Transform,
    utils::default,
};
use strum_macros::EnumString;

use crate::{app_state::{AppState, AppStateSet}, clickable::Clickable, Size};

#[derive(Component, Clone, Default)]
struct Cell {
    cell_type: Option<CellType>,
    revealed: bool,
}
impl Cell {
    const SIZE: f32 = 40.;

    pub fn spawn(
        &self,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        translation: Vec3,
    ) {
        let mut cell = commands.spawn((
            self.clone(),
            Mesh2d(meshes.add(Rectangle::from_length(Self::SIZE))),
            MeshMaterial2d(materials.add(Color::from(GREEN_400))),
            Transform::from_translation(translation),
        ));
        cell.with_children(|cell| {
            if let Some(cell_type) = &self.cell_type {
                cell_type.spawn(cell, asset_server);
            }
            if !self.revealed {
                CellCover::spawn(cell, meshes, materials);
            }
        });
    }
}
impl From<char> for Cell {
    fn from(character: char) -> Self {
        Self {
            cell_type: match character {
                // Fox Species
                'V' => Some(CellType::PawPrint(FoxSpecies::Vulpes)),
                'C' => Some(CellType::PawPrint(FoxSpecies::Corsac)),
                // Obstacles
                's' => Some(CellType::Obstacle(ObstacleType::Stones)),
                _ => None,
            },
            ..default()
        }
    }
}
#[derive(Debug, EnumString, Clone)]
enum CellType {
    PawPrint(FoxSpecies),
    Obstacle(ObstacleType),
}
impl CellType {
    fn spawn(&self, cell: &mut ChildBuilder<'_>, asset_server: &Res<AssetServer>) {
        cell.spawn((
            Sprite {
                image: asset_server.load(format!("images/{self:?}.png")),
                custom_size: Some(Vec2::splat(Cell::SIZE)),
                ..default()
            },
            Size(Vec2::splat(Cell::SIZE)),
            Transform::from_translation(Vec3::Z),
        ));
    }
}
#[derive(Component)]
struct CellCover;
impl CellCover {
    fn spawn(
        cell: &mut ChildBuilder<'_>,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) {
        cell.spawn((
            Self,
            Clickable::new_event(CellCoverClickedEvent),
            Size(Vec2::splat(Cell::SIZE)),
            Mesh2d(meshes.add(Rectangle::from_length(Cell::SIZE))),
            MeshMaterial2d(materials.add(Color::from(GREEN_800))),
            Transform::from_translation(2. * Vec3::Z),
        ));
    }
}
#[derive(Debug, Default, Clone)]
enum FoxSpecies {
    #[default]
    Vulpes,
    Corsac,
}
#[derive(Debug, Default, Clone)]
enum ObstacleType {
    #[default]
    Stones,
}
#[derive(Resource, Default)]
struct Level(usize);
mod level_statics {
    use once_cell::sync::Lazy;

    pub static LEVEL_LAYOUTS: Lazy<Vec<Vec<&str>>> = Lazy::new(|| {
        vec![vec![
            "C     ", //
            "  V   ", //
            "   C  ", //
            "    s ", //
            "      ", //
        ]]
    });
}
#[derive(Event, Debug)]
struct CellCoverClickedEvent(Entity);

pub(crate) struct SearchPlugin;
impl Plugin for SearchPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Level::default())
            .add_event::<CellCoverClickedEvent>()
            .add_systems(OnEnter(AppState::Search), search_startup.after(AppStateSet))
            .add_systems(Update, (reveal_cell).run_if(in_state(AppState::Search)));
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
    let level_layout = &level_statics::LEVEL_LAYOUTS[level.0];
    let height = level_layout.len();
    let start_y = (height - 1) as f32 / 2.;
    for (y, row) in level_layout.iter().enumerate() {
        for (x, character) in row.chars().enumerate() {
            let mut cell = Cell::from(character);
            cell.revealed = x == 0;
            cell.spawn(
                &mut commands,
                &asset_server,
                &mut meshes,
                &mut materials,
                Vec3 {
                    x: x as f32 * Cell::SIZE,
                    y: (start_y - y as f32) * Cell::SIZE,
                    z: 0.,
                },
            );
        }
    }
}
#[allow(clippy::needless_pass_by_value)]
fn reveal_cell(
    mut commands: Commands,
    mut cell_cover_clicked: EventReader<CellCoverClickedEvent>,
    cell_covers_q: Query<(&Parent, Entity), With<CellCover>>,
    mut cells_q: Query<&mut Cell>,
) {
    for ev in cell_cover_clicked.read() {
        if let Ok((cell_cover_parent, cell_cover)) = cell_covers_q.get(ev.0) {
            if let Ok(mut cell) = cells_q.get_mut(cell_cover_parent.get()) {
                cell.revealed = true;
            }
            commands.entity(cell_cover).despawn_recursive();
        }
    }
}
