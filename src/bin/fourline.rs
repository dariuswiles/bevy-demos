/// A basic implementation of the classic table-top strategy game that consists of trying to get a
/// line of 4 pieces vertically, horizontally or diagonally before your computer opponent. The
/// computer randomly picks a column for each of its moves, so it shouldn't be hard to beat!
use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowResolution};

const WINDOW_WIDTH: f32 = 700.0;
const WINDOW_HEIGHT: f32 = 700.0;
const BOARD_COLUMNS: usize = 7;
const BOARD_ROWS: usize = 6;
const SPRITE_FILENAME: &str = "sprites/fourline.png";
const SPRITE_WIDTH: usize = 80;
const SPRITE_HEIGHT: usize = 80;

type Cell = Option<Player>;

/// A label that is applied to the primary camera to make it more convenient to reference.
#[derive(Component)]
struct PrimaryCamera;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Player {
    Computer,
    Human,
}

/// Indicates if a game is in progress or is over.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, States)]
enum GameState {
    GameOver,
    #[default]
    Playing,
}

/// Used when a game ends to indicate who won, or if the game was drawn.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum GameOutcome {
    ComputerWon,
    Draw,
    HumanWon,
}

/// Game data. `cells` is an array where the index of the bottom-left cell is 0, the cell to
/// its right is 1, and the cell above is BOARD_COLUMNS. The last cell is the top-right cell, which
/// has an index of BOARD_ROWS * BOARD_COLUMNS - 1.
#[derive(Resource)]
struct GameData {
    cells: [Cell; BOARD_COLUMNS * BOARD_ROWS],
    texture_atlas: Handle<TextureAtlas>,
    current_player: Option<Player>,
    game_outcome: Option<GameOutcome>,
}

impl GameData {
    /// Creates a new game consisting of an empty board, the given texture atlas to use to draw
    /// the board and player pieces, and which player has the first turn.
    fn new(texture_atlas: Handle<TextureAtlas>, starting_player: Player) -> Self {
        Self {
            cells: [None; BOARD_COLUMNS * BOARD_ROWS],
            texture_atlas,
            current_player: Some(starting_player),
            game_outcome: None,
        }
    }

    /// Adds a new piece for the given player in the lowest empty cell in `col`. On success, returns
    /// the row index of the new piece with an `Ok`, or `Err` if `col` is full.
    fn make_move(&mut self, column: usize, player: Player) -> Result<usize, ()> {
        if let Some(vacant_row) = self.lowest_vacant_row(column) {
            self.cells[BOARD_COLUMNS * vacant_row + column] = Some(player);
            Result::Ok(vacant_row)
        } else {
            Result::Err(())
        }
    }

    /// Returns the index of the row nearest the bottom of the game board that has a vacant cell in
    /// the given `col`. Returns `None` if the column is full.
    fn lowest_vacant_row(&self, col: usize) -> Option<usize> {
        for row in 0..BOARD_ROWS {
            if self.cells[row * BOARD_COLUMNS + col] == None {
                return Some(row);
            }
        }
        None
    }

    /// Returns `true` if the piece at the cell defined by `column` and `row` is part of a line of 4
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
        let played_piece = self.cells[row * BOARD_COLUMNS + col];
        if played_piece == None {
            return false;
        }

        for (c_disp, r_disp) in &[(0, 1), (1, 1), (1, 0), (1, -1)] {
            let mut line_length = 0;

            for i in -3..=3 {
                let c = col as i8 + i * c_disp;
                let r = row as i8 + i * r_disp;

                if (c < 0) || (c >= 7) || (r < 0) || (r >= 6) {
                    continue;
                }

                if self.cells[(r * BOARD_COLUMNS as i8 + c) as usize] == played_piece {
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
        for col in 0..BOARD_COLUMNS {
            if self.cells[(BOARD_ROWS - 1) * BOARD_COLUMNS + col] == None {
                return false;
            }
        }
        true
    }
}

/// Creates a 2D camera and loads a texture atlas file that contains a tile used for each cell of
/// the board, and a tile for a player piece.
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // Creates a 2D camera and adds a component named `PrimaryCamera` to make it more convenient to
    // reference.
    commands
        .spawn(Camera2dBundle::default())
        .insert(PrimaryCamera);

    let texture_handle = asset_server.load(SPRITE_FILENAME);
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(SPRITE_WIDTH as f32, SPRITE_HEIGHT as f32),
        2, // Number of columns of individual tiles in our image file loaded from disk.
        1, // Number of rows of individual tiles in our image file loaded from disk.
        None, // There is no padding between tiles.
        None, // The first tile is not offset from the top-left corner.
    );
    let texture_atlas_handle: Handle<_> = texture_atlases.add(texture_atlas);

    commands.insert_resource(GameData::new(texture_atlas_handle.clone(), Player::Human));

    create_board(
        &mut commands,
        texture_atlas_handle.clone(),
        TextureAtlasSprite {
            index: 0,
            ..Default::default()
        },
    );
}

/// Creates the graphics for an empty board using the graphics tile passed.
fn create_board(
    commands: &mut Commands,
    texture_atlas_handle: Handle<TextureAtlas>,
    board_sprite: TextureAtlasSprite,
) {
    let x_left = ((BOARD_COLUMNS - 1) * SPRITE_WIDTH) as f32 / -2.0;
    let y_bottom = ((BOARD_ROWS - 1) * SPRITE_HEIGHT) as f32 / -2.0;

    for r in 0..BOARD_ROWS {
        for c in 0..BOARD_COLUMNS {
            let position = Vec3::new(
                x_left + (SPRITE_WIDTH * c) as f32,
                y_bottom + (SPRITE_HEIGHT * r) as f32,
                1.0,
            );

            commands.spawn(SpriteSheetBundle {
                sprite: board_sprite.clone(),
                texture_atlas: texture_atlas_handle.clone(),
                transform: Transform::from_translation(position),
                ..Default::default()
            });
        }
    }
}

/// The main game loop that is called each frame to make a move on behalf of the human player or
/// computer, and then check to see if that move ends the game.
fn game_loop(
    mut mouse_button_input: ResMut<Input<MouseButton>>,
    mut primary_query: Query<&mut Window, With<PrimaryWindow>>,
    camera: Query<&Transform, With<PrimaryCamera>>,
    mut commands: Commands,
    mut gd: ResMut<GameData>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    match gd.current_player {
        Some(Player::Human) => {
            let mut primary_window = primary_query
                .get_single_mut()
                .expect("Internal error: cannot locate primary window");

            let result = human_move(
                &mut mouse_button_input,
                &mut primary_window,
                &camera,
                &mut commands,
                &mut gd,
            );

            if result == true {
                gd.current_player = Some(Player::Computer);
            }
        }
        Some(Player::Computer) => {
            computer_move(&mut commands, &mut gd);
            gd.current_player = Some(Player::Human);
        }
        None => {}
    }

    let game_status = is_game_over(&gd);
    if game_status.is_some() {
        gd.current_player = None;
        gd.game_outcome = game_status;
        next_state.set(GameState::GameOver);
    }
}

/// If the user clicked on a column of the board, attempts to play a piece there. Returns `true` if
/// a move was successfully made, `false` otherwise.
fn human_move(
    mouse_button_input: &mut ResMut<Input<MouseButton>>,
    primary_window: &mut Window,
    camera: &Query<&Transform, With<PrimaryCamera>>,
    commands: &mut Commands,
    gd: &mut ResMut<GameData>,
) -> bool {
    let mut result = false;

    if mouse_button_input.just_pressed(MouseButton::Left) {
        if let Some(pos) = primary_window.cursor_position() {
            if let Ok(selected_column) =
                convert_mouse_position_to_column_id(&primary_window, &camera.single(), pos)
            {
                if let Result::Ok(r) = gd.make_move(selected_column, Player::Human) {
                    add_piece_to_board(&gd, commands, selected_column, r, Player::Human);
                    result = true;
                }
            }
        }
        mouse_button_input.reset(MouseButton::Left);
    }

    result
}

/// Adds a piece to the graphical game board at coordinates `col` and `row`, and using the color
/// of the current player, as defined in `gd`.
fn add_piece_to_board(
    gd: &GameData,
    commands: &mut Commands,
    column: usize,
    row: usize,
    player: Player,
) {
    let x_offset = (BOARD_COLUMNS - 1) as f32 / 2.0;
    let y_offset = (BOARD_ROWS - 1) as f32 / 2.0;

    let x = (column as f32 - x_offset) * SPRITE_WIDTH as f32;
    let y = (row as f32 - y_offset) * SPRITE_HEIGHT as f32;

    let mut color = Color::BLUE;
    if player == Player::Computer {
        color = Color::RED;
    }

    commands.spawn(SpriteSheetBundle {
        sprite: TextureAtlasSprite {
            index: 1,
            color,
            ..Default::default()
        },
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
    let pos_distance_x = pos_world.x + (BOARD_COLUMNS as f32 / 2.0) * SPRITE_WIDTH as f32;
    let pos_col = (pos_distance_x / SPRITE_WIDTH as f32) as usize;

    if (pos_distance_x > 0.0) & (pos_col < BOARD_COLUMNS) {
        return Ok(pos_col);
    }
    Err(())
}

/// Makes a move in a random column.
fn computer_move(
    commands: &mut Commands,
    gd: &mut ResMut<GameData>,
    // mut next_state: ResMut<NextState<GameState>>,
) {
    loop {
        let selected_column = fastrand::u8(0..BOARD_COLUMNS as u8) as usize;

        if let Result::Ok(r) = gd.make_move(selected_column, Player::Computer) {
            add_piece_to_board(&gd, commands, selected_column, r, Player::Computer);
            break;
        }
    }
}

/// Determines if the game has been won. by the move at `col` and `row`, or if the game is drawn
/// because the board is full. If so, sets the `state` to indicate which player won or that the
/// game is drawn and returns `true`. If no-one has won and the game is not drawn, returns `false`.
fn is_game_over(gd: &GameData) -> Option<GameOutcome> {
    for r in 0..BOARD_ROWS {
        for c in 0..BOARD_COLUMNS {
            let p = gd.cells[r * BOARD_COLUMNS + c];

            if p.is_some() {
                // total_pieces += 1;
                if gd.is_winning_move(c, r) {
                    if p == Some(Player::Computer) {
                        return Some(GameOutcome::ComputerWon);
                    } else {
                        return Some(GameOutcome::HumanWon);
                    }
                }
            }
        }
    }

    if gd.is_board_full() {
        return Some(GameOutcome::Draw);
    }

    None
}

/// Displays a message at the top of the play area indicating the game has been won by the stated
/// player, or drawn.
fn display_game_outcome(
    commands: Commands,
    gd: ResMut<GameData>,
    asset_server: Res<AssetServer>,
) {
    match gd.game_outcome.unwrap() {
        GameOutcome::ComputerWon => {
            display_text(commands, asset_server, "Computer wins");
        }
        GameOutcome::Draw => {
            display_text(commands, asset_server, "Game drawn");
        }
        GameOutcome::HumanWon => {
            display_text(commands, asset_server, "You win!");
        }
    }
}

/// Displays the given text at the top-center of the window.
fn display_text(mut commands: Commands, asset_server: Res<AssetServer>, s: &str) {
    commands
        .spawn(NodeBundle {
            background_color: Color::rgba(0.0, 0.0, 0.0, 0.0).into(),
            style: Style {
                justify_content: JustifyContent::Center,
                margin: UiRect {
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
            parent.spawn(TextBundle {
                text: Text::from_section(
                    s,
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 80.0,
                        color: Color::rgb(0.6, 0.6, 1.0),
                    },
                ),
                ..Default::default()
            });
        });
}

fn main() {
    let wd = Window {
        resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
        title: String::from("Fourline"),
        ..Default::default()
    };

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(wd),
            ..default()
        }))
        .add_state::<GameState>()
        .add_systems(Startup, setup)
        .add_systems(Update, game_loop.run_if(in_state(GameState::Playing)))
        .add_systems(OnEnter(GameState::GameOver), display_game_outcome)
        .run();
}
