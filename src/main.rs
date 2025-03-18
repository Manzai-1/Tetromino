use macroquad::{color, prelude::*};
use std::{thread, time};
use ::rand::prelude::*;

const GAME_WIDTH: f32 = 300.0;
const GAME_HEIGHT: f32 = GAME_WIDTH * 2.0;

const BLOCK_GRID_X: f32 = 10.0;
const BLOCK_GRID_Y: f32 = 20.0;

const BLOCK_WIDTH: f32 = GAME_WIDTH / BLOCK_GRID_X;
const BLOCK_HEIGHT: f32 = GAME_HEIGHT / BLOCK_GRID_Y;

const UI_BLOCKS_X: i32 = 2; 
const UI_BLOCKS_Y: i32 = 2; 

const UI_WIDTH: f32 = BLOCK_WIDTH * UI_BLOCKS_X as f32;
const UI_HEIGHT: f32 = (BLOCK_HEIGHT * UI_BLOCKS_Y as f32) * 2.0;

const WINDOW_WIDTH: i32 = GAME_WIDTH as i32 + UI_WIDTH as i32;
const WINDOW_HEIGHT: i32 = GAME_HEIGHT as i32 + (UI_HEIGHT as i32 * 2);

const WINDOW_BLOCKS_X: i32 = WINDOW_WIDTH / BLOCK_WIDTH as i32;
const WINDOW_BLOCKS_Y: i32 = WINDOW_HEIGHT / BLOCK_HEIGHT as i32;

const GAME_BOARD_LENGTH: usize = BLOCK_GRID_X as usize * BLOCK_GRID_Y as usize;

const SHAPE_BLOCKS_X: usize = 3;
const SHAPE_BLOCKS_Y: usize = 3;

struct BlockColor {
    dark: Color,
    medium: Color,
    light: Color
}

const BLOCK_COLORS: [BlockColor; 5] = [
    BlockColor{
        dark: BLACK,
        medium: BLACK,
        light: BLACK
    },
    BlockColor{
        dark: DARKBLUE,
        medium: BLUE,
        light: SKYBLUE
    },
    BlockColor{
        dark: DARKGREEN,
        medium: LIME,
        light: GREEN
    },
    BlockColor{
        dark: ORANGE,
        medium: GOLD,
        light: YELLOW
    },
    BlockColor{
        dark: DARKBLUE,
        medium: PURPLE,
        light: PINK
    }
];

#[derive(Clone)]
struct TetroShape {
    shape: [i32; 9]
}

struct Tetromino {
    pos_x: f32,
    pos_y: f32,
    shape: TetroShape,
    color_id: usize
}

const TETRO_SHAPES: [TetroShape; 5] = [
    TetroShape {
        shape: 
        [0,0,0,
        0,1,0,
        1,1,1]
    },
    TetroShape {
        shape: 
        [1,0,0,
        1,1,0,
        0,1,0]
    },
    TetroShape {
        shape: 
        [0,1,0,
        1,1,0,
        1,0,0]
    },
    TetroShape {
        shape: 
        [0,0,0,
        1,1,0,
        1,1,0]
    },
    TetroShape {
        shape: 
        [0,1,0,
        0,1,0,
        0,1,0]
    }
];


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
    let mut skipDelay = false;
    let delay = time::Duration::from_millis(500);
    let mut tetromino: Tetromino = generate_tetromino();
    //let mut game_board: [i32; GAME_BOARD_LENGTH] = [0; GAME_BOARD_LENGTH]; 

    loop {
        if skipDelay {
            skipDelay = false;
            println!("RESET SkipDelay");
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            let (mouse_x,mouse_y) = mouse_position();
            let btn_id = mouse_event(mouse_x, mouse_y);
            
            match btn_id {
                1 => println!("ROTATE LEFT"),
                2 => {
                    tetromino.pos_x -= BLOCK_WIDTH;
                    skipDelay = true;
                },
                3 => println!("MOVE DOWN"),
                4 => {
                    tetromino.pos_x += BLOCK_WIDTH;
                    skipDelay = true;
                },
                5 => println!("ROTATE RIGHT"),
                _ => println!("NO BUTTON CLICKED"),
            }
        }

        if !skipDelay {
            tetromino.pos_y += BLOCK_HEIGHT;
            thread::sleep(delay);
        }

        clear_background(BLACK);
        draw_ui();
        draw_buttons();
        draw_score(50);
        
        render_game(&tetromino);

        next_frame().await
    }
}

fn render_game(tetromino: &Tetromino) {
    for i in 0..SHAPE_BLOCKS_X * SHAPE_BLOCKS_Y {
        if tetromino.shape.shape[i] > 0 {
            let pos_x = tetromino.pos_x + ((i as f32 % SHAPE_BLOCKS_X as f32) * BLOCK_WIDTH);
            let pos_y = tetromino.pos_y + ((i as i32 / SHAPE_BLOCKS_Y as i32) as f32 * BLOCK_HEIGHT);

            draw_block(pos_x, pos_y, tetromino.color_id);
        }
    }
}

fn generate_tetromino () -> Tetromino {
    let mut rng = ::rand::rng();
    let color_id = rng.random::<u32>() % 3;
    let shape_id = rng.random::<u32>() % 5;

    println!("COLOR_ID: {}", color_id);
    println!("SHAPE_ID: {}", shape_id);
    
    Tetromino{
        pos_x: BLOCK_WIDTH * 2.0,
        pos_y: UI_HEIGHT,
        shape: TETRO_SHAPES[shape_id as usize].clone(),
        color_id: (color_id + 1) as usize
    }
}

fn mouse_event(x: f32, y: f32) -> i32 {

    let btn_range_y1 = WINDOW_HEIGHT as f32 - (UI_HEIGHT * 1.5);
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
    let ui_color_id = 4;

    for y in 0..WINDOW_BLOCKS_Y {
        draw_block(0.0, y as f32 * BLOCK_HEIGHT, ui_color_id);
        draw_block(WINDOW_WIDTH as f32 - BLOCK_WIDTH, y as f32 * BLOCK_HEIGHT, ui_color_id);
    }

    for n in 1..WINDOW_BLOCKS_X -1{
        let pos_x: f32 = n as f32 * BLOCK_WIDTH;
        draw_block(pos_x, 0.0, ui_color_id);
        draw_block(pos_x, WINDOW_HEIGHT as f32 - BLOCK_HEIGHT, ui_color_id);
        draw_block(pos_x, UI_HEIGHT - BLOCK_HEIGHT, ui_color_id);
        draw_block(pos_x, WINDOW_HEIGHT as f32 - UI_HEIGHT, ui_color_id);
    }
}

fn draw_buttons() {
    let btn_pos_y: f32 = WINDOW_HEIGHT as f32 - (UI_HEIGHT - BLOCK_HEIGHT);
    let btn_pos_x: f32 = UI_WIDTH / 2.0;

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
    

fn draw_block(pos_x:f32, pos_y:f32, col_id: usize) {
    draw_rectangle(pos_x +2.0, pos_y +2.0, BLOCK_WIDTH - 4.0, BLOCK_HEIGHT - 4.0, BLOCK_COLORS[col_id].dark);
    draw_rectangle(pos_x +6.0, pos_y +6.0, BLOCK_WIDTH - 12.0, BLOCK_HEIGHT - 12.0, BLOCK_COLORS[col_id].medium);
    draw_rectangle(pos_x +12.0, pos_y +12.0, BLOCK_WIDTH - 24.0, BLOCK_HEIGHT - 24.0, BLOCK_COLORS[col_id].light);
}

fn draw_button(pos_x:f32, pos_y:f32, text: &str){
    let button_height = BLOCK_HEIGHT * 2.0;
    let button_width = BLOCK_WIDTH * 2.0;

    let text_pos_y = WINDOW_HEIGHT as f32 - (UI_HEIGHT / 2.0) + (BLOCK_WIDTH * 0.2);
    
    draw_rectangle(pos_x +2.0, pos_y +2.0, button_width - 4.0, button_height - 4.0, DARKGRAY);
    draw_rectangle(pos_x +6.0, pos_y +6.0, button_width - 12.0, button_height - 12.0, GRAY);
    draw_text(&text, pos_x + (BLOCK_WIDTH * 0.2), text_pos_y, BLOCK_HEIGHT * 0.9, BLACK);
}
