/// Load a sprite sheet (which Bevy prefers to call a texture atlas) from a file, and display its
/// individual sprites. Create a `GameData` resource to hold game state. Monitor for keyboard
/// input for a number key between 1 and 7 (inclusive) being pressed and output the current
/// board state on each press.

use bevy::prelude::*;
use std::fmt;

const NUM_COLUMNS: usize = 7;
const NUM_ROWS: usize = 6;
const SPRITE_FILENAME: &str = "sprites/fourline.png";
const SPRITE_WIDTH: usize = 80;
const SPRITE_HEIGHT: usize = 80;

type Cell = Option<PlayerColor>;


/// Used purely to label the camera so it is easier to find when converting the cursor position
/// between coordinate systems.
struct PrimaryCamera;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum PlayerColor {
    Blue,
    Red,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum GameState {
    HumanToMove,
    ComputerToMove,
    GameWon,
    GameDrawn,
}


/// The GameData. `cells` is an array where the index of the bottom-left cell is 0, the cell to
/// its right is 1, and the cell above is NUM_COLUMNS. The last cell is the top-right cell, which
/// has an index of NUM_ROWS * NUM_COLUMNS - 1.
struct GameData {
    cells: [Cell; NUM_COLUMNS * NUM_ROWS],
    next_player_color: PlayerColor,
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
                        PlayerColor::Blue => {
                            output.push_str("B ");
                        }
                        PlayerColor::Red => {
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
            next_player_color: PlayerColor::Blue,
            texture_atlas,
        }
    }

    /// Add a new piece for the next player in the lowest empty cell in `col`. On success, return
    /// the row index of the new piece with an `Ok`, or `Err` if `col` is full.
    fn make_move(&mut self, col: usize) -> Result<usize, ()> {
        assert!(col < NUM_COLUMNS);

        if let Some(vacant_row) = self.lowest_vacant_row(col) {
            self.cells[NUM_COLUMNS * vacant_row + col] = Some(self.next_player_color);
            Result::Ok(vacant_row)
        } else {
            Result::Err(())
        }
    }

    /// Return the index of the row nearest the bottom of the game board that has a vacant cell in
    /// the given `col`. Return `None` if the column is full.
    fn lowest_vacant_row(&self, col: usize) -> Option<usize> {
        assert!(col < NUM_COLUMNS);

        for row in 0..NUM_ROWS {
            if self.cells[row * NUM_COLUMNS + col] == None {
                return Some(row);
            }
        }

        None
    }

    /// Return `true` if the piece at the cell defined by `column` and `row` is part of a line of 4
    /// pieces for the same player.
    ///
    /// # Panics
    ///
    /// Panics if either `col` or `row` are not a valid column or row reference respectively.
    //
    // The outer 'for' loop uses an array of (column displacement, row displacement) pairs. These
    // are used in the inner loop to modify the `col` and `row` passed as parameters to examine the
    // pieces in one direction. For example, the first outer loop uses a column displacement of 0
    // and a vertical displacement of 1, thereby checking the pieces vertically. The `c` and `r`
    // variables are allowed to extend beyond valid column and row ranges, but any invalid values
    // are discarded before being used to perform lookups. They are signed so that they can extend
    // in the negative direction.
    fn is_winning_move(&self, col: usize, row: usize) -> bool {
        assert!(col < NUM_COLUMNS);
        assert!(row < NUM_ROWS);

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
//                 println!("\ti = {}\t(c = {}, r = {}, line_length = {})", i, c, r, line_length);
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


fn make_move(column: usize, gd: &mut ResMut<GameData>) -> Result<usize, ()> {
//     println!("Next move being made in column {:#?}", column);
//     println!("Game board before\n{:#?}", **gd);

    let move_result = gd.make_move(column);
    if let Err(()) = move_result {
        println!("make_move failed because column {} is full", column);
//     } else {
//         println!("Game board after\n{:#?}", **gd);
    }
    move_result
}



/// Wait for the user to click on a column of the board and attempt to play a piece there.
fn main_loop(
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
//             println!("User clicked at raw position {}, {}", pos.x, pos.y);

            if let Ok(selected_column) = convert_mouse_position_to_column_id(
                &primary_window,
                &camera.single().unwrap(),
                pos,
            ) {
//                 println!("\twhich translates to column {}", selected_column);

                if let Result::Ok(r) = make_move(selected_column, &mut gd) {
//                     println!("Move was successfully made in row {}", r);

                    add_piece_to_board(&gd, &mut commands, selected_column, r);

                    if gd.is_winning_move(selected_column, r) {
                        if gd.cells[r * NUM_COLUMNS + selected_column].unwrap() == PlayerColor::Blue {
                            println!("Blue wins!");
                        } else {
                            println!("Red wins!");
                        }

                        if state.current() != &GameState::GameWon {
                            if let Err(_) = state.set(GameState::GameWon) {
                                println!("Failed to change to 'GameWon' state");
                            }
                            return;
                        }
                    }

                    if gd.is_board_full() {
                        println!("Game drawn");
                        if let Err(_) = state.set(GameState::GameDrawn) {
                            println!("Failed to change to 'GameDrawn' state");
                        }
                        return;
                    }


                    if gd.next_player_color == PlayerColor::Blue {
                        gd.next_player_color = PlayerColor::Red;
                        println!("Switching to Red player");
                        if let Err(_) = state.set(GameState::ComputerToMove) {
                            println!("Failed to change to 'ComputerToMove' state");
                        }
                    } else {
                        gd.next_player_color = PlayerColor::Blue;
                        println!("Switching to Blue player");
                        if let Err(_) = state.set(GameState::HumanToMove) {
                            println!("Failed to change to 'HumanToMove' state");
                        }
                    }
                }
            }
        }

        mouse_button_input.reset(MouseButton::Left);
    }
}



/// Adds a piece to the graphical game board at coordinations `col` and `row`, and using the color
/// of the current player, as defined in `gd`.
fn add_piece_to_board(gd: &GameData, commands: &mut Commands, col: usize, row: usize) {
    let x_offset = (NUM_COLUMNS - 1) as f32 / 2.0;
    let y_offset = (NUM_ROWS - 1) as f32 / 2.0;

    let x = (col as f32 - x_offset) * SPRITE_WIDTH as f32;
    let y = (row as f32 - y_offset) * SPRITE_HEIGHT as f32;
    let color;

    if gd.next_player_color == PlayerColor::Blue {
        color = Color::BLUE;
    } else {
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


/// Wait for the user to click on a column of the board and attempt to play a piece there.
fn computer_move(
    mut commands: Commands,
    mut gd: ResMut<GameData>,
    mut state: ResMut<State<GameState>>,
) {
    let selected_column = fastrand::u8(0..7) as usize;

    if let Result::Ok(r) = make_move(selected_column, &mut gd) {
                    println!("Move was successfully made in row {}", r);

        add_piece_to_board(&gd, &mut commands, selected_column, r);

        if gd.is_winning_move(selected_column, r) {
            if gd.cells[r * NUM_COLUMNS + selected_column].unwrap() == PlayerColor::Blue {
                println!("Blue wins!");
            } else {
                println!("Red wins!");
            }

            if state.current() != &GameState::GameWon {
                if let Err(_) = state.set(GameState::GameWon) {
                    println!("Failed to change to 'GameWon' state");
                }
                return;
            }
        }

        if gd.is_board_full() {
            println!("Game drawn");
            if let Err(_) = state.set(GameState::GameDrawn) {
                println!("Failed to change to 'GameDrawn' state");
            }
            return;
        }

        if gd.next_player_color == PlayerColor::Blue {
            gd.next_player_color = PlayerColor::Red;
            println!("Switching to Red player");
            if let Err(_) = state.set(GameState::ComputerToMove) {
                println!("Failed to change to 'ComputerToMove' state");
            }
        } else {
            gd.next_player_color = PlayerColor::Blue;
            println!("Switching to Blue player");
            if let Err(_) = state.set(GameState::HumanToMove) {
                println!("Failed to change to 'HumanToMove' state");
            }
        }
    }
}


fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_state(GameState::HumanToMove)
        .add_system_set(SystemSet::on_update(GameState::HumanToMove).with_system(main_loop.system()))
        .add_system_set(SystemSet::on_update(GameState::ComputerToMove).with_system(computer_move.system()))
        .run();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lowest_vacant_row_in_empty_grid() {
        let mut gd = GameData::default();

        assert_eq!(Some(0), gd.lowest_vacant_row(0));
        assert_eq!(Some(0), gd.lowest_vacant_row(4));
        assert_eq!(Some(0), gd.lowest_vacant_row(NUM_COLUMNS - 1));
    }

    #[test]
    fn lowest_vacant_row_in_partial_column() {
        let mut gd = GameData::default();

        gd.next_player_color = PlayerColor::Blue;
        gd.make_move(2);
        assert_eq!(Some(1), gd.lowest_vacant_row(2));

        gd.next_player_color = PlayerColor::Red;
        gd.make_move(2);
        assert_eq!(Some(2), gd.lowest_vacant_row(2));
    }

    #[test]
    fn horizontal_winner() {
        let mut gd = GameData {
            ..Default::default()
        };
        assert_eq!(false, gd.is_winning_move(3, 0));

        gd.cells[2] = Some(PlayerColor::Red);
        gd.cells[3] = Some(PlayerColor::Red);
        gd.cells[4] = Some(PlayerColor::Red);
        gd.cells[5] = Some(PlayerColor::Red);
        assert_eq!(true, gd.is_winning_move(3, 0));

        gd.cells[4] = Some(PlayerColor::Blue);
        assert_eq!(false, gd.is_winning_move(3, 0));
    }

    #[test]
    fn vertical_winner() {

        let mut gd = GameData {
            ..Default::default()
        };
        assert_eq!(false, gd.is_winning_move(2, 0));

        gd.cells[2] = Some(PlayerColor::Red);
        gd.cells[NUM_COLUMNS + 2] = Some(PlayerColor::Red);
        gd.cells[2 * NUM_COLUMNS + 2] = Some(PlayerColor::Red);
        gd.cells[3 * NUM_COLUMNS + 2] = Some(PlayerColor::Red);
        assert_eq!(true, gd.is_winning_move(2, 0));

        gd.cells[2 * NUM_COLUMNS + 2] = Some(PlayerColor::Blue);
        assert_eq!(false, gd.is_winning_move(2, 0));
    }
}
