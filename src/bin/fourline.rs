/// Fourline - win by making a line vertically, horizontally or diagonally before your computer
/// opponent. The computer randomly picks a column for each move, so shouldn't be hard to beat!

use bevy::prelude::*;
use std::fmt;


const WINDOW_WIDTH: f32 = 700.0;
const WINDOW_HEIGHT: f32 = 700.0;
const NUM_COLUMNS: usize = 7;
const NUM_ROWS: usize = 6;
const SPRITE_FILENAME: &str = "sprites/fourline.png";
const SPRITE_WIDTH: usize = 80;
const SPRITE_HEIGHT: usize = 80;

type Cell = Option<Player>;


/// Used to label the primary camera so it is easier to find when converting the cursor position
/// between coordinate systems.
struct PrimaryCamera;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Player {
    Human,
    Computer,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum GameState {
    HumanMove,
    ComputerMove,
    HumanWon,
    ComputerWon,
    GameDrawn,
}


/// Game data. `cells` is an array where the index of the bottom-left cell is 0, the cell to
/// its right is 1, and the cell above is NUM_COLUMNS. The last cell is the top-right cell, which
/// has an index of NUM_ROWS * NUM_COLUMNS - 1.
struct GameData {
    cells: [Cell; NUM_COLUMNS * NUM_ROWS],
    texture_atlas: Handle<TextureAtlas>,
}

impl fmt::Debug for GameData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = String::new();
        for r in (0..NUM_ROWS).rev() {
            for c in 0..NUM_COLUMNS {
                let cell = self.cells[r * NUM_COLUMNS + c];

                if let Some(token) = cell {
                    match token {
                        Player::Human => {
                            output.push_str("B ");
                        }
                        Player::Computer => {
                            output.push_str("R ");
                        }
                    }
                } else {
                    output.push_str("_ ");
                }
            }
            output.push_str("\n");
        }
        f.write_str(&output)
    }
}

impl GameData {
    fn new(
        texture_atlas: Handle<TextureAtlas>,
    ) -> Self {
        Self {
            cells: [None; NUM_COLUMNS * NUM_ROWS],
            texture_atlas,
        }
    }

    /// Add a new piece for the next player in the lowest empty cell in `col`. On success, return
    /// the row index of the new piece with an `Ok`, or `Err` if `col` is full.
    fn make_move(&mut self, column: usize, color: Player) -> Result<usize, ()> {
        if let Some(vacant_row) = self.lowest_vacant_row(column) {
            self.cells[NUM_COLUMNS * vacant_row + column] = Some(color);
            Result::Ok(vacant_row)
        } else {
            Result::Err(())
        }
    }

    /// Return the index of the row nearest the bottom of the game board that has a vacant cell in
    /// the given `col`. Return `None` if the column is full.
    fn lowest_vacant_row(&self, col: usize) -> Option<usize> {
        for row in 0..NUM_ROWS {
            if self.cells[row * NUM_COLUMNS + col] == None {
                return Some(row);
            }
        }
        None
    }

    /// Return `true` if the piece at the cell defined by `column` and `row` is part of a line of 4
    /// pieces for the same player.
    //
    // The outer 'for' loop uses an array of (column displacement, row displacement) pairs. These
    // are used in the inner loop to modify the `col` and `row` passed as parameters to examine the
    // pieces in one direction. For example, the first outer loop uses a column displacement of 0
    // and a vertical displacement of 1, thereby checking the pieces vertically. The `c` and `r`
    // variables are allowed to extend beyond valid column and row ranges, but any invalid values
    // are discarded before being used to perform lookups. They are signed so that they can extend
    // in the negative direction.
    fn is_winning_move(&self, col: usize, row: usize) -> bool {
        let played_piece = self.cells[row * NUM_COLUMNS + col];
        if played_piece == None { return false; }

        for (c_disp, r_disp) in &[(0, 1), (1, 1), (1, 0), (1, -1)] {
            let mut line_length = 0;

            for i in -3..=3 {
                let c = col as i8 + i * c_disp;
                let r = row as i8 + i * r_disp;

                if (c < 0) || (c >= 7) || (r < 0) || (r >= 6) {
                    continue;
                }

                if self.cells[(r * NUM_COLUMNS as i8 + c) as usize] == played_piece {
                    line_length += 1;
                    if line_length == 4 {
                        return true;
                    }
                } else {
                    line_length = 0;
                }
            }
        }
        false
    }

    /// Returns `true` if every cell in the top row is full, i.e., no further moves are possible.
    fn is_board_full(&self) -> bool {
        for col in 0..NUM_COLUMNS {
            if self.cells[(NUM_ROWS - 1) * NUM_COLUMNS + col] == None {
                return false;
            }
        }
        true
    }
}


fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // Create a 2D camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d()).insert(PrimaryCamera);

    let texture_handle = asset_server.load(SPRITE_FILENAME);
    let texture_atlas = TextureAtlas::from_grid(texture_handle,
        Vec2::new(SPRITE_WIDTH as f32, SPRITE_HEIGHT as f32), 2, 1
    );
    let texture_atlas_handle: Handle<_> = texture_atlases.add(texture_atlas);

    commands.insert_resource(GameData::new(
        texture_atlas_handle.clone(),
    ));

    create_board(&mut commands,
        texture_atlas_handle.clone(),
        TextureAtlasSprite { index: 0, ..Default::default() },
    );
}


fn create_board(
    commands: &mut Commands,
    texture_atlas_handle: Handle<TextureAtlas>,
    board_sprite: TextureAtlasSprite,
) {
    let x_left = ((NUM_COLUMNS - 1) * SPRITE_WIDTH) as f32 / -2.0;
    let y_bottom = ((NUM_ROWS - 1) * SPRITE_HEIGHT) as f32 / -2.0;

    for r in 0..NUM_ROWS {
        for c in 0..NUM_COLUMNS {
            let position = Vec3::new(
                x_left + (SPRITE_WIDTH * c) as f32,
                y_bottom + (SPRITE_HEIGHT * r) as f32,
                1.0
            );

            commands.spawn_bundle(SpriteSheetBundle {
                sprite: board_sprite.clone(),
                texture_atlas: texture_atlas_handle.clone(),
                transform: Transform::from_translation(position),
                ..Default::default()
            });
        }
    }
}


/// Wait for the user to click on a column of the board and attempt to play a piece there.
fn human_move(
    mut mouse_button_input: ResMut<Input<MouseButton>>,
    windows: Res<Windows>,
    camera: Query<&Transform, With<PrimaryCamera>>,
    mut commands: Commands,
    mut gd: ResMut<GameData>,
    mut state: ResMut<State<GameState>>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        let primary_window = windows.get_primary().unwrap();

        if let Some(pos) = primary_window.cursor_position() {
            if let Ok(selected_column) = convert_mouse_position_to_column_id(
                &primary_window,
                &camera.single().unwrap(),
                pos,
            ) {
                if let Result::Ok(r) = gd.make_move(selected_column, player_color_from_state(&state)) {
                    add_piece_to_board(&gd, &mut commands, selected_column, r, player_color_from_state(&state));
                    if !is_game_over(&mut gd, &mut state, selected_column, r) {
                        state.set(GameState::ComputerMove).unwrap();
                    }
                }
            }
        }
        mouse_button_input.reset(MouseButton::Left);
    }
}


fn human_won(
    commands: Commands,
    asset_server: Res<AssetServer>,
    materials: ResMut<Assets<ColorMaterial>>
) {
    display_text(commands, asset_server, materials, "You win!");
}

fn computer_won(
    commands: Commands,
    asset_server: Res<AssetServer>,
    materials: ResMut<Assets<ColorMaterial>>
) {
    display_text(commands, asset_server, materials, "Computer wins");
}

fn game_drawn(
    commands: Commands,
    asset_server: Res<AssetServer>,
    materials: ResMut<Assets<ColorMaterial>>
) {
    display_text(commands, asset_server, materials, "Game drawn");
}


/// Adds a piece to the graphical game board at coordinations `col` and `row`, and using the color
/// of the current player, as defined in `gd`.
fn add_piece_to_board(
    gd: &GameData,
    commands: &mut Commands,
    column: usize,
    row: usize,
    player_color: Player,
) {
    let x_offset = (NUM_COLUMNS - 1) as f32 / 2.0;
    let y_offset = (NUM_ROWS - 1) as f32 / 2.0;

    let x = (column as f32 - x_offset) * SPRITE_WIDTH as f32;
    let y = (row as f32 - y_offset) * SPRITE_HEIGHT as f32;

    let mut color = Color::BLUE;
    if player_color == Player::Computer {
        color = Color::RED;
    }

    commands.spawn_bundle(SpriteSheetBundle {
        sprite: TextureAtlasSprite { index: 1, color, ..Default::default() },
        texture_atlas: gd.texture_atlas.clone(),
        transform: Transform::from_translation(Vec3::new(x, y, 0.0)),
        ..Default::default()
    });
}



/// Converts the raw window position in `pos` to a column id, where the leftmost column is 0. If
/// `pos` is horizontally outside the board, return `Err`.
fn convert_mouse_position_to_column_id(
    window: &Window,
    camera_transform: &Transform,
    pos: Vec2,
) -> Result<usize, ()> {
    let window_size = Vec2::new(window.width() as f32, window.height() as f32);

    // Convert from the coordinate system used for mouse positions, where the lower-left of
    // the window is 0,0 to world coordinates, i.e., the 2D orthographic system where 0,0
    // is the center of the window.
    let p = pos - window_size / 2.0;
    let pos_world = camera_transform.compute_matrix() * p.extend(0.0).extend(1.0);

    // Convert this 'world' position to the corresponding board column, where 0 is on the left.
    let pos_distance_x = pos_world.x + (NUM_COLUMNS as f32 / 2.0) * SPRITE_WIDTH as f32;
    let pos_col = (pos_distance_x / SPRITE_WIDTH as f32) as usize;

    if (pos_distance_x > 0.0) & (pos_col < NUM_COLUMNS) {
        return Ok(pos_col);
    }
    Err(())
}


/// Make a move in a random column.
fn computer_move(
    mut commands: Commands,
    mut gd: ResMut<GameData>,
    mut state: ResMut<State<GameState>>,
) {
    loop {
        let selected_column = fastrand::u8(0..NUM_COLUMNS as u8) as usize;

        if let Result::Ok(r) = gd.make_move(selected_column, player_color_from_state(&state)) {
            add_piece_to_board(&gd, &mut commands, selected_column, r,
                player_color_from_state(&state)
            );
            if !is_game_over(&mut gd, &mut state, selected_column, r) {
                state.set(GameState::HumanMove).unwrap();
            }
            break;
        }
    }
}


/// Determines if the game has been won by the move at `col` and `row`, or if the game is drawn
/// because the board is full. If so, sets the `state` to indicate which player won or that the
/// game is drawn and returns `true`. If no-one has won and the game is not drawn, returns `false`.
fn is_game_over(
    gd: &mut ResMut<GameData>,
    state: &mut ResMut<State<GameState>>,
    col: usize,
    row: usize
) -> bool {
    if gd.is_winning_move(col, row) {
        if gd.cells[row * NUM_COLUMNS + col].unwrap() == Player::Human {
            if state.current() != &GameState::HumanWon {
                state.set(GameState::HumanWon).unwrap();
                return true;
            }
        } else {
            if state.current() != &GameState::ComputerWon {
                state.set(GameState::ComputerWon).unwrap();
                return true;
            }
        }
    }
    if gd.is_board_full() {
        state.set(GameState::GameDrawn).unwrap();
        return true;
    }
    false
}


/// Returns the Player associated with the given `State`.
///
/// # Panics
///
/// Panics if `State` is not `HumanMove` or `ComputerMove`.
fn player_color_from_state(state: &State<GameState>) -> Player {
    match state.current() {
        GameState::HumanMove => { Player::Human },
        GameState::ComputerMove => { Player::Computer },
        _ => { panic!("Current state is not associated with a player color"); }
    }
}


/// Display the given text near the top-left of the window.
fn display_text(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    s: &str,
) {
    commands.spawn_bundle(UiCameraBundle::default());

    commands
        .spawn_bundle(NodeBundle {
            material: materials.add(Color::rgba(0.0, 0.0, 0.0, 0.0).into()),
            style: Style {
                justify_content: JustifyContent::Center,
                margin: Rect {
                    top: Val::Px(10.0),
                    bottom: Val::Auto,
                    left: Val::Auto,
                    right: Val::Auto,
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    s,
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 80.0,
                        color: Color::rgb(0.6, 0.6, 1.0),
                    },
                    Default::default()
                ),
                ..Default::default()
            });
        });
}


fn main() {
    let wd = WindowDescriptor {
        width: WINDOW_WIDTH,
        height: WINDOW_HEIGHT,
        title: String::from("Fourline"),
        ..Default::default()
    };

    App::build()
        .insert_resource(wd)
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_state(GameState::HumanMove)
        .add_system_set(SystemSet::on_update(GameState::HumanMove)
            .with_system(human_move.system())
        )
        .add_system_set(SystemSet::on_enter(GameState::ComputerMove)
            .with_system(computer_move.system())
        )
        .add_system_set(SystemSet::on_enter(GameState::HumanWon)
            .with_system(human_won.system())
        )
        .add_system_set(SystemSet::on_enter(GameState::ComputerWon)
            .with_system(computer_won.system())
        )
        .add_system_set(SystemSet::on_enter(GameState::GameDrawn)
            .with_system(game_drawn.system())
        )
        .run();
}
