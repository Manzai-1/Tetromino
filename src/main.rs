use macroquad::prelude::*;
use ::rand::prelude::*;

const GAME_WIDTH: f32 = 300.0;
const GAME_HEIGHT: f32 = GAME_WIDTH * 2.0;

const GAME_BLOCKS_X: usize = 10;
const GAME_BLOCKS_Y: usize = 20;

const BLOCK_WIDTH: f32 = GAME_WIDTH / GAME_BLOCKS_X as f32;
const BLOCK_HEIGHT: f32 = GAME_HEIGHT / GAME_BLOCKS_Y as f32;

const UI_BLOCK_WIDTH: usize = 1;
const UI_BLOCK_HEIGHT: usize = 4;

const UI_WIDTH: f32 = BLOCK_WIDTH * UI_BLOCK_WIDTH as f32;
const UI_HEIGHT: f32 = BLOCK_HEIGHT * UI_BLOCK_HEIGHT as f32;

const WINDOW_WIDTH: i32 = GAME_WIDTH as i32 + (UI_WIDTH * 2.0) as i32;
const WINDOW_HEIGHT: i32 = GAME_HEIGHT as i32 + (UI_HEIGHT as i32 * 2);

const WINDOW_BLOCKS_X: i32 = WINDOW_WIDTH / BLOCK_WIDTH as i32;
const WINDOW_BLOCKS_Y: i32 = WINDOW_HEIGHT / BLOCK_HEIGHT as i32;

const TETRO_BLOCKS_X: usize = 3;
const TETRO_BLOCKS_Y: usize = 3;

struct BlockColor {
    dark: Color,
    medium: Color,
    light: Color
}

#[derive(Clone, Copy, PartialEq)]
enum BlockType {
    Empty,
    Blue,
    Green,
    Orange,
    UI
}

impl BlockType {
    fn get_color(self) -> BlockColor {
        match self {
            BlockType::Empty => BlockColor { dark: BLACK, medium: BLACK, light: BLACK },
            BlockType::Blue  => BlockColor { dark: MAROON, medium: RED, light: PINK },
            BlockType::Green => BlockColor { dark: DARKGREEN, medium: LIME, light: GREEN },
            BlockType::Orange => BlockColor { dark: ORANGE, medium: GOLD, light: YELLOW },
            BlockType::UI => BlockColor { dark: DARKBLUE, medium: BLUE, light: SKYBLUE },
        }
    }

    fn get_block_type(n: usize) -> BlockType {
        match n {
            0 => BlockType::Blue,
            1 => BlockType::Green,
            2 => BlockType::Orange,
            _ => BlockType::Empty
        }
    }
}

#[derive(Clone, Copy)]
struct TetroShape {
    shape: [[bool; TETRO_BLOCKS_X]; TETRO_BLOCKS_Y]
}

#[derive(Copy, Clone)]
enum TetrominoType {
    T, L, S, O, I
}

impl TetrominoType {
    fn get_shape(self) -> TetroShape {
        match self{
            TetrominoType::T => TetroShape { shape: [
                [false, false, false],
                [false, true,  false],
                [true,  true,  true],
            ] },
            TetrominoType::L => TetroShape { shape: [
                [false, true, false],
                [false, true, false],
                [false, true, true],
            ] },
            TetrominoType::S => TetroShape { shape: [
                [false, true, false],
                [false, true, true],
                [false, false, true],
            ] },
            TetrominoType::O => TetroShape { shape: [
                [false, false, false],
                [false, true, true],
                [false, true, true],
            ] },
            TetrominoType::I => TetroShape { shape: [
                [false, true, false],
                [false, true, false],
                [false, true, false],
            ] },
        }
    }

    fn get_tetro_type (n: usize) -> TetrominoType{
        match n {
            0 => TetrominoType::T,
            1 => TetrominoType::L,
            2 => TetrominoType::S,
            3 => TetrominoType::O,
            _ => TetrominoType::I,
        }
    }
}

#[derive(Clone, Copy)]
struct Tetromino {
    pos_x: i32,
    pos_y: i32,
    tetro_type: TetroShape,
    tetro_style: BlockType
}

#[derive(Clone, Copy)]
enum TetrominoAction {
    RotateLeft,
    RotateRight,
    MoveLeft,
    MoveRight,
    MoveDown
}

impl TetrominoAction {
    fn get_value(self) -> i32 {
        match self {
            TetrominoAction::RotateLeft => -1,
            TetrominoAction::RotateRight  => 1,
            TetrominoAction::MoveLeft => -1,
            TetrominoAction::MoveRight => 1,
            TetrominoAction::MoveDown => 1,
        }
    }
}

struct GameBoard {
    board: [[BlockType; GAME_BLOCKS_X]; GAME_BLOCKS_Y],
    score: i32,
    running: bool
}



fn window_conf() -> Conf {
    Conf {
        window_title: "Tetromino".to_owned(),
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut fps_limit = 60;
    let mut loop_index = 0;

    let mut tetromino: Tetromino = generate_tetromino();

    let mut game_board: GameBoard = GameBoard{
        board: [[BlockType::Empty; GAME_BLOCKS_X]; GAME_BLOCKS_Y],
        score: 0,
        running: true
    };

    loop {

        if is_mouse_button_pressed(MouseButton::Left) {
            let (mouse_x,mouse_y) = mouse_position();
            let btn_id = mouse_event(mouse_x, mouse_y);
            
            match btn_id {
                1 => {
                    tetromino = tetromino_action(tetromino, TetrominoAction::RotateLeft, &mut game_board);
                },
                2 => {
                    tetromino = tetromino_action(tetromino, TetrominoAction::MoveLeft, &mut game_board);
                },
                3 => {
                    tetromino = tetromino_action(tetromino, TetrominoAction::MoveDown, &mut game_board);
                },
                4 => {
                    tetromino = tetromino_action(tetromino, TetrominoAction::MoveRight, &mut game_board);
                },
                5 => {
                    tetromino = tetromino_action(tetromino, TetrominoAction::RotateRight, &mut game_board);
                },
                _ => println!("NO BUTTON CLICKED"),
            }
        }

        if loop_index == fps_limit {
            tetromino = tetromino_action(tetromino, TetrominoAction::MoveDown, &mut game_board);
            loop_index = 0;
        }

        loop_index += 1;

        draw_ui();
        draw_buttons();
        draw_score(game_board.score);
        
        render_game(&tetromino, game_board.board);

        next_frame().await
    }
}


fn check_game_board(game_board: &mut GameBoard){
    let mut y = GAME_BLOCKS_Y -1;
    let mut points = 0;

    while y > 0{
        if !game_board.board[y].contains(&BlockType::Empty){
            points += 10;
            for y2 in (0..y+1).rev() {
                if y2 > 0 {
                    game_board.board[y2] = game_board.board[y2-1];
                }
            }
        }else {
            y = y -1;
        }
    }

    game_board.score += points;
}


fn tetromino_action (
    mut tetromino: Tetromino, 
    action: TetrominoAction, 
    game_board: &mut GameBoard
) -> Tetromino {
    match action {
        TetrominoAction::RotateLeft | TetrominoAction::RotateRight=> {
            let new_shape: TetroShape = rotate_tetromino(
                tetromino.tetro_type, 
                action.get_value());

            if detect_collission(
                new_shape, 
                tetromino.pos_x, 
                tetromino.pos_y, 
                0, 
                0, 
                &mut game_board.board
            ){
                println!("CANT PERFORM ROTATION")
            }else {
                tetromino.tetro_type = new_shape;
            }

        },
        TetrominoAction::MoveLeft | TetrominoAction::MoveRight => {
            if detect_collission(
                tetromino.tetro_type, 
                tetromino.pos_x, 
                tetromino.pos_y, 
                action.get_value(), 
                0, 
                &mut game_board.board
            ){
                println!("CANT PERFORM MOVE")
            }else {
                tetromino.pos_x += action.get_value();
            }
        },
        TetrominoAction::MoveDown => {
            if detect_collission(
                tetromino.tetro_type, 
                tetromino.pos_x, 
                tetromino.pos_y, 
                0, 
                action.get_value(), 
                &mut game_board.board
            ){
                handle_collission(&mut game_board.board, tetromino);
                check_game_board(game_board);
                tetromino = generate_tetromino();
            }else {
                tetromino.pos_y += action.get_value();
            }
        },
    }
    tetromino
}


fn rotate_tetromino (tetro_shape: TetroShape, direction: i32) -> TetroShape {
    let mut new_shape = TetroShape{shape: [[false; TETRO_BLOCKS_X]; TETRO_BLOCKS_Y]};
    
    for y in 0..TETRO_BLOCKS_Y {
        for x in 0..TETRO_BLOCKS_X {
            if direction == 1 {
                new_shape.shape[x][(TETRO_BLOCKS_Y - 1) - y] = tetro_shape.shape[y][x];
            } else {
                new_shape.shape[(TETRO_BLOCKS_X - 1) - x][y] = tetro_shape.shape[y][x];
            }
        }
    }
    
    new_shape
}


fn detect_collission(tetro_shape: TetroShape, pos_x: i32, pos_y: i32, add_x: i32, add_y: i32, game_board: &mut [[BlockType; GAME_BLOCKS_X]; GAME_BLOCKS_Y]) -> bool {
    for (y, row) in tetro_shape.shape.iter().enumerate() {
        for (x, &block) in row.iter().enumerate() {
            if block {
                let new_x = pos_x + x as i32 + add_x;
                let new_y = pos_y + y as i32 + add_y;

                if  new_x < 0  || new_x > (GAME_BLOCKS_X - 1) as i32 {
                    return true;
                } else if new_y > (GAME_BLOCKS_Y - 1) as i32 {
                    return true;
                } else if !matches!(game_board[new_y as usize][new_x as usize], BlockType::Empty) {
                    return true;
                }
            }
        }
    }

    return false;
}


fn handle_collission(game_board:&mut [[BlockType; GAME_BLOCKS_X]; GAME_BLOCKS_Y], tetromino: Tetromino) {
    for (y, row) in tetromino.tetro_type.shape.iter().enumerate() {
        for (x, &block) in row.iter().enumerate() {
            if block {
                game_board[(tetromino.pos_y + y as i32) as usize][(tetromino.pos_x + x as i32) as usize] = tetromino.tetro_style;
            }
        }
    }
}



fn generate_tetromino () -> Tetromino {
    let mut rng = ::rand::rng();
    let color_id: usize = (rng.random::<u32>() % 3) as usize;
    let shape_id: usize = (rng.random::<u32>() % 5) as usize;
    let tetro_shape: TetroShape = TetrominoType::get_shape(TetrominoType::get_tetro_type(shape_id));

    Tetromino{
        pos_x: GAME_BLOCKS_X as i32 / 3,
        pos_y: 0,
        tetro_type: tetro_shape,
        tetro_style: BlockType::get_block_type(color_id)
    }
}


fn render_game(tetromino: &Tetromino, game_board: [[BlockType; GAME_BLOCKS_X]; GAME_BLOCKS_Y]) {
    for (y, row) in game_board.iter().enumerate() {
        for (x, &block) in row.iter().enumerate() {
            let pos_x: f32 = (x + UI_BLOCK_WIDTH) as f32 * BLOCK_WIDTH; 
            let pos_y = (y + UI_BLOCK_HEIGHT) as f32 * BLOCK_HEIGHT; 

            draw_block(pos_x, pos_y, block);
        }
    }
    
    for (y, row) in tetromino.tetro_type.shape.iter().enumerate() {
        for (x, &block) in row.iter().enumerate() {
            if block {
                let pos_x = (tetromino.pos_x as f32 + (x + UI_BLOCK_WIDTH) as f32) * BLOCK_WIDTH; //tetromino.pos_x + (x as f32 * BLOCK_WIDTH);
                let pos_y = (tetromino.pos_y as f32 + (y + UI_BLOCK_HEIGHT) as f32) * BLOCK_HEIGHT;//tetromino.pos_y + (y as f32 * BLOCK_HEIGHT);
                draw_block(pos_x, pos_y, tetromino.tetro_style);
            }
        }
    }
}

fn mouse_event(x: f32, y: f32) -> i32 {

    let btn_range_y1 = WINDOW_HEIGHT as f32 - (UI_HEIGHT);
    let btn_range_y2 = WINDOW_HEIGHT as f32 - (BLOCK_HEIGHT);

    if y > btn_range_y1 && y < btn_range_y2 {
        if x > BLOCK_WIDTH && x < BLOCK_WIDTH * 3.0 {
            1
        } else if x > BLOCK_WIDTH && x < BLOCK_WIDTH * 5.0 {
            2
        } else if x > BLOCK_WIDTH && x < BLOCK_WIDTH * 7.0 {
            3
        } else if x > BLOCK_WIDTH && x < BLOCK_WIDTH * 9.0 {
            4
        } else if x > BLOCK_WIDTH && x < BLOCK_WIDTH * 11.0 {
            5
        } else {
            0
        }
    }else {
        0
    }
}


fn draw_ui() {
    for y in 0..WINDOW_BLOCKS_Y {
        draw_block(0.0, y as f32 * BLOCK_HEIGHT, BlockType::UI);
        draw_block(WINDOW_WIDTH as f32 - BLOCK_WIDTH, y as f32 * BLOCK_HEIGHT, BlockType::UI);
    }

    for n in 1..WINDOW_BLOCKS_X -1{
        let pos_x: f32 = n as f32 * BLOCK_WIDTH;
        draw_block(pos_x, 0.0, BlockType::UI);
        draw_block(pos_x, WINDOW_HEIGHT as f32 - BLOCK_HEIGHT, BlockType::UI);
        draw_block(pos_x, UI_HEIGHT - BLOCK_HEIGHT, BlockType::UI);
        draw_block(pos_x, WINDOW_HEIGHT as f32 - UI_HEIGHT, BlockType::UI);
    }
}

fn draw_buttons() {
    let btn_pos_y: f32 = WINDOW_HEIGHT as f32 - (UI_HEIGHT - BLOCK_HEIGHT);
    let btn_pos_x: f32 = UI_WIDTH;

    draw_button(btn_pos_x, btn_pos_y, "-180");
    draw_button(btn_pos_x + (BLOCK_WIDTH * 2.0), btn_pos_y, "<--");
    draw_button(btn_pos_x + (BLOCK_WIDTH * 4.0), btn_pos_y, "DOWN");
    draw_button(btn_pos_x + (BLOCK_WIDTH * 6.0), btn_pos_y, "-->");
    draw_button(btn_pos_x + (BLOCK_WIDTH * 8.0), btn_pos_y, "+180");
}

fn draw_score(score: i32) {
    let score: String = format!("{} P", score);
    let font_size: f32 = UI_HEIGHT / 2.0;
    let points_length = score.chars().count();

    let points_pos_x = (WINDOW_WIDTH as f32 / 2.0) - (points_length as f32 * BLOCK_WIDTH / 2.25);
    let points_pos_y = (UI_HEIGHT as f32 / 2.0) + (font_size / 4.0);
    
    draw_text(&score, points_pos_x, points_pos_y, font_size, SKYBLUE);
}
    

fn draw_block(pos_x:f32, pos_y:f32, style: BlockType) {
    draw_rectangle(pos_x +2.0, pos_y +2.0, BLOCK_WIDTH - 4.0, BLOCK_HEIGHT - 4.0, BlockType::get_color(style).dark);
    draw_rectangle(pos_x +6.0, pos_y +6.0, BLOCK_WIDTH - 12.0, BLOCK_HEIGHT - 12.0, BlockType::get_color(style).medium);
    draw_rectangle(pos_x +12.0, pos_y +12.0, BLOCK_WIDTH - 24.0, BLOCK_HEIGHT - 24.0, BlockType::get_color(style).light);
}

fn draw_button(pos_x:f32, pos_y:f32, text: &str){
    let button_height = BLOCK_HEIGHT * 2.0;
    let button_width = BLOCK_WIDTH * 2.0;

    let text_pos_y = WINDOW_HEIGHT as f32 - (UI_HEIGHT / 2.0) + (BLOCK_WIDTH * 0.2);
    
    draw_rectangle(pos_x +2.0, pos_y +2.0, button_width - 4.0, button_height - 4.0, DARKGREEN);
    draw_rectangle(pos_x +6.0, pos_y +6.0, button_width - 12.0, button_height - 12.0, GREEN);
    draw_text(&text, pos_x + (BLOCK_WIDTH * 0.2), text_pos_y, BLOCK_HEIGHT * 0.9, BLACK);
}
