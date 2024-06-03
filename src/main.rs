mod figures;
use figures::initial_fig;
use macroquad::prelude::*;

const N_SQUARES: usize = 32;

#[derive(Debug, Clone, PartialEq)]
enum Cell {
    Alive,
    Dead,
}

impl Cell {
    fn rules(&self, n: usize) -> Cell {
        if *self == Cell::Alive {
            if n == 2 || n == 3 {
                Cell::Alive
            } else {
                Cell::Dead
            }
        } else if n == 3 {
            Cell::Alive
        } else {
            Cell::Dead
        }
    }
}

fn create_config() -> Conf {
    Conf {
        window_title: String::from("Game of Life"),
        window_width: 600,
        window_height: 600,
        ..Default::default()
    }
}

fn draw_board(sq_size: f32, buffer: &[Vec<Cell>]) {
    for x in 0..N_SQUARES {
        for y in 0..N_SQUARES {
            draw_rectangle(
                x as f32 * sq_size,
                y as f32 * sq_size,
                sq_size,
                sq_size,
                match buffer[x][y] {
                    Cell::Alive => BLACK,
                    Cell::Dead => WHITE,
                },
            );
        }
    }

    for i in 1..N_SQUARES {
        draw_line(
            0.,
            sq_size * i as f32,
            screen_width(),
            sq_size * i as f32,
            2.,
            LIGHTGRAY,
        );
        draw_line(
            sq_size * i as f32,
            0.,
            sq_size * i as f32,
            screen_height(),
            2.,
            LIGHTGRAY,
        );
    }
}

fn module(a: i32, b: i32) -> usize {
    ((a % b + b) % b) as usize
}

fn count_neighbours(front: &[Vec<Cell>], x: i32, y: i32) -> usize {
    let mut count = 0;
    for dx in -1..=1_i32 {
        for dy in -1..=1_i32 {
            if !(dx == 0 && dy == 0) {
                let x = module(x + dx, front[0].len() as i32);
                let y = module(y + dy, front.len() as i32);
                if front[x][y] == Cell::Alive {
                    count += 1;
                }
            }
        }
    }

    count
}

#[macroquad::main(create_config())]
async fn main() {
    let sq_size = screen_width() / N_SQUARES as f32;
    // Use initial_custom to specify your own starting postion.
    let mut front = initial_fig(N_SQUARES, "pulsar", 6);
    let mut buffer = front.clone();
    let mut time = get_time();
    let mut pause = false;

    let mut update_time = 1.0;

    loop {
        if is_key_pressed(KeyCode::Space) {
            pause = !pause;
        }

        if is_key_pressed(KeyCode::Enter) && update_time >= 0.2 {
            update_time -= 0.1;
        }

        if is_key_pressed(KeyCode::Backspace) && update_time <= 1.0 {
            update_time += 0.1;
        }

        if is_mouse_button_down(MouseButton::Left) {
            let pos = mouse_position();
            let col = (pos.0 / sq_size).floor() as usize;
            let row = (pos.1 / sq_size).floor() as usize;
            buffer[col][row] = Cell::Alive;
        }

        if is_mouse_button_down(MouseButton::Right) {
            let pos = mouse_position();
            let col = (pos.0 / sq_size).floor() as usize;
            let row = (pos.1 / sq_size).floor() as usize;
            buffer[col][row] = Cell::Dead;
        }
        draw_board(sq_size, &buffer);

        // Update board
        if get_time() - time >= update_time && !pause {
            time = get_time();
            for x in 0..N_SQUARES {
                for y in 0..N_SQUARES {
                    let n_neighbours = count_neighbours(&front, x as i32, y as i32);
                    buffer[x][y] = front[x][y].rules(n_neighbours);
                }
            }
        }
        front.clone_from(&buffer);
        next_frame().await
    }
}
