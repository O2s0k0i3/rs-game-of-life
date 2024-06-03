use crate::Cell;
use std::collections::HashMap;

pub fn initial_fig(n_squares: usize, fig: &str, offset: usize) -> Vec<Vec<Cell>> {
    let mut figures: HashMap<&str, Vec<(usize, usize)>> = HashMap::new();
    figures.insert("glider", vec![(1, 0), (2, 1), (0, 2), (1, 2), (2, 2)]);
    figures.insert("block", vec![(2, 2), (2, 3), (3, 2), (3, 3)]);
    figures.insert(
        "bee-hive",
        vec![(1, 2), (2, 1), (3, 1), (4, 2), (2, 3), (3, 3)],
    );
    figures.insert(
        "loaf",
        vec![(2, 1), (3, 1), (4, 2), (4, 3), (1, 2), (2, 3), (3, 4)],
    );
    figures.insert("boat", vec![(1, 1), (1, 2), (2, 1), (3, 2), (2, 3)]);
    figures.insert("tub", vec![(1, 2), (2, 1), (3, 2), (2, 3)]);
    figures.insert("blinker", vec![(1, 2), (2, 2), (3, 2)]);
    figures.insert("toad", vec![(1, 3), (2, 2), (3, 2), (4, 2), (2, 3), (3, 3)]);
    figures.insert(
        "pentadecathlon",
        vec![
            (1, 2),
            (2, 2),
            (3, 1),
            (3, 3),
            (4, 2),
            (5, 2),
            (6, 2),
            (7, 2),
            (8, 1),
            (8, 3),
            (9, 2),
            (10, 2),
        ],
    );
    figures.insert(
        "beacon",
        vec![(1, 1), (2, 1), (1, 2), (3, 4), (4, 4), (4, 3)],
    );
    figures.insert(
        "pulsar",
        vec![
            (2, 4),
            (2, 5),
            (2, 6),
            (4, 2),
            (5, 2),
            (6, 2),
            (7, 4),
            (7, 5),
            (7, 6),
            (4, 7),
            (5, 7),
            (6, 7),
            //
            (9, 4),
            (9, 5),
            (9, 6),
            (10, 2),
            (11, 2),
            (12, 2),
            (7, 10),
            (7, 11),
            (7, 12),
            (4, 14),
            (5, 14),
            (6, 14),
            //
            (2, 10),
            (2, 11),
            (2, 12),
            (4, 9),
            (5, 9),
            (6, 9),
            (14, 4),
            (14, 5),
            (14, 6),
            (10, 7),
            (11, 7),
            (12, 7),
            //
            (9, 10),
            (9, 11),
            (9, 12),
            (10, 9),
            (11, 9),
            (12, 9),
            (14, 10),
            (14, 11),
            (14, 12),
            (10, 14),
            (11, 14),
            (12, 14),
        ],
    );

    let mut front = vec![vec![Cell::Dead; n_squares]; n_squares];
    for (x, y) in &figures[fig] {
        front[(*x + offset) % n_squares][(*y + offset) % n_squares] = Cell::Alive;
    }

    front
}

pub fn initial_custom(n_squares: usize, coords: Vec<(usize, usize)>) -> Vec<Vec<Cell>> {
    let mut front = vec![vec![Cell::Dead; n_squares]; n_squares];
    for (x, y) in coords {
        front[x][y] = Cell::Alive;
    }
    front
}
