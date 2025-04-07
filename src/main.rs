use bevy::{prelude::*, sprite::Wireframe2dPlugin, window::EnabledButtons};

const WIDTH: f32 = 800.;
const HEIGHT: f32 = 600.;

#[derive(Resource)]
struct ChessBoardPos {
    bot_left: (f32, f32),
    size: f32,
    square_size: f32,
    square_pos: Vec<(f32, f32)>, // Row major matrix
}

impl ChessBoardPos {
    fn new(bot_left: (f32, f32), size: f32) -> Self {
        let square_size = size / 8.;
        let square_pos = (0..64).map(|idx| {
            let i = idx % 8;
            let j = idx / 8;
            (bot_left.1 + (i as f32) * square_size, bot_left.0 + (j as f32) * square_size)
        }).collect();
        Self {
            bot_left,
            size,
            square_size,
            square_pos,
        }
    }
}

#[derive(Resource)]
struct BoardState {
    squares: Vec<Square> // Row major matrix
}

impl Default for BoardState {
    fn default() -> Self {
        let base_line = vec![Rook, Knight, Bishop, Queen, King, Bishop, Knight, Rook];
        let pawn_line: Vec<Piece> = std::iter::repeat(Pawn).take(8).collect();

        let all_empty = std::iter::repeat(Square::Empty).take(32).collect::<Vec<_>>();

        let board = base_line.iter()
            .map(|p| Square::Occupied(*p, White))
            .chain(pawn_line.iter().map(|p| Square::Occupied(*p, White)))
            .chain(all_empty)
            .chain(pawn_line.iter().map(|p| Square::Occupied(*p, Black)))
            .chain(base_line.iter().map(|p| Square::Occupied(*p, Black)))
            .collect::<Vec<_>>();

        Self {
            squares: board,
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Piece {
    Pawn,
    Bishop,
    Knight,
    Rook,
    Queen,
    King,
}
use Piece::*;

#[derive(Debug, Copy, Clone)]
enum Team {
    White,
    Black,
}
use Team::*;

#[derive(Debug, Copy, Clone)]
enum Square {
    Empty,
    Occupied(Piece, Team),
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Quantum Chess".into(),
                name: Some("qhess".into()),
                resolution: (WIDTH, HEIGHT).into(),
                window_theme: Some(bevy::window::WindowTheme::Dark),
                enabled_buttons: EnabledButtons {
                    maximize: false,
                    ..Default::default()
                },
                ..default()
            }),
            ..default()
        }))
        .add_plugins(Wireframe2dPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands:  Commands, asset_server: Res<AssetServer>) {
}
