use bevy::{prelude::*, sprite::{Anchor, Wireframe2dPlugin}, window::EnabledButtons};

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
            (bot_left.1 + (i as f32) * square_size + square_size / 2.,
                bot_left.0 + (j as f32) * square_size + square_size / 2.)
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
            .map(|p| Square::Occupied(*p, Probability::Full, White))
            .chain(pawn_line.iter().map(|p| Square::Occupied(*p, Probability::Full, White)))
            .chain(all_empty)
            .chain(pawn_line.iter().map(|p| Square::Occupied(*p, Probability::Full, Black)))
            .chain(base_line.iter().map(|p| Square::Occupied(*p, Probability::Full, Black)))
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
enum Probability {
    Full,
    Half,
}

#[derive(Debug, Copy, Clone, Component)]
enum Square {
    Empty,
    Occupied(Piece, Probability, Team),
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
        .insert_resource(ChessBoardPos::new((-300., -300.), HEIGHT))
        .add_systems(Startup, draw_board)
        .run();
}

fn draw_board(mut commands:  Commands, board_pos: Res<ChessBoardPos>,
    mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn(Camera2d);

    let board = BoardState::default();
    let mut squares = vec![];

    for _ in board_pos.square_pos.iter() {
        squares.push(meshes.add(Rectangle::new(board_pos.square_size, board_pos.square_size)));
    }

    for (i, shape) in squares.into_iter().enumerate() {
        let color = if (i % 8 + i / 8) % 2 == 0 {
            Color::srgb(0., 0., 0.)
        } else {
            Color::srgb(1., 1., 1.)
        };

        commands.spawn((
            Mesh2d(shape),
            MeshMaterial2d(materials.add(color)),
            Transform::from_xyz(
                board_pos.square_pos[i].0,
                board_pos.square_pos[i].1,
                0.
            ),
            board.squares[i],
            Anchor::Center,
        ));
    }

    commands.spawn((Mesh2d(meshes.add(Circle::new(10.))),
            MeshMaterial2d(materials.add(Color::srgb(1., 0., 0.))),
            Transform::from_xyz(0., 0.,0.)));
}
