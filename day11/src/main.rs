use itertools::Itertools;
use std::io::{self, Read};

type Coord = (u32, u32);

struct Grid {
    side: usize,
    pls: Vec<i8>,
}

fn get_power_level(coord: &Coord, serial_n: i32) -> i8 {
    let x = coord.0 as i32;
    let y = coord.1 as i32;

    let id = x + 10;
    let pl = id * y;
    let pl = pl + serial_n;
    let pl = pl * id;
    let pl = (pl / 100 % 10) as i8;
    let pl = pl - 5;

    pl
}

fn init_grid(side: usize, serial_n: i32) -> Grid {
    let range = || (1..=side as u32);
    let pls = range()
        .cartesian_product(range())
        .map(|c| get_power_level(&c, serial_n))
        .collect();

    Grid { side, pls }
}

fn compute_sub_squares(grid: &Grid) -> Grid {
    let side = grid.side - 2;
    let pls = &grid.pls;
    let get = |x, y| pls[grid.side * (x - 1) + y - 1];
    let agg_pls = (1..=side)
        .cartesian_product(1..=side)
        .map(|(x, y)| {
            get(x, y)
                + get(x, y + 1)
                + get(x, y + 2)
                + get(x + 1, y)
                + get(x + 1, y + 1)
                + get(x + 1, y + 2)
                + get(x + 2, y)
                + get(x + 2, y + 1)
                + get(x + 2, y + 2)
        })
        .collect();

    Grid { side, pls: agg_pls }
}

fn compute_sub_squares_from_prev(base_grid: &Grid, prev_grid: &Grid) -> Grid {
    let side = prev_grid.side - 1;
    let prev_square_size = base_grid.side - side;

    let get = |g: &Grid, (x, y)| g.pls[g.side * (x - 1) + y - 1];
    let pls = (1..=prev_grid.side - 1)
        .cartesian_product(1..=prev_grid.side - 1)
        .map(|(x, y)| {
            // power level of the previous square
            // plus the line under it, the column at its right
            // and the cell in the corner
            get(prev_grid, (x, y))
                + get(base_grid, (x + prev_square_size, y + prev_square_size))
                + (0..prev_square_size)
                    .map(|offset| {
                        get(base_grid, (x + prev_square_size, y + offset))
                            + get(base_grid, (x + offset, y + prev_square_size))
                    })
                    .sum::<i8>()
        })
        .collect();

    Grid { side, pls }
}

fn get_max_square(grid: &Grid) -> (i8, Coord) {
    let (total, idx) = grid
        .pls
        .iter()
        .enumerate()
        .map(|(i, v)| (v, i))
        .max()
        .expect("Canot find square");

    let side = grid.side;
    let x = (idx / side) + 1;
    let y = (idx % side) + 1;

    (*total, (x as u32, y as u32))
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;
    let serial_n = input.trim().parse::<i32>().expect("Serial number");

    let side = 300;
    let power_levels_grid = init_grid(side, serial_n);
    let squares_grid = compute_sub_squares(&power_levels_grid);
    let (total, coord) = get_max_square(&squares_grid);

    println!("{:?} with a total of {}", coord, total);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{
        compute_sub_squares, compute_sub_squares_from_prev, get_max_square, get_power_level,
        init_grid,
    };

    #[test]
    fn test_get_power_level() {
        assert_eq!(get_power_level(&(3, 5), 8), 4);
        assert_eq!(get_power_level(&(122, 79), 57), -5);
        assert_eq!(get_power_level(&(217, 196), 39), 0);
        assert_eq!(get_power_level(&(101, 153), 71), 4);
    }

    #[test]
    fn test_init_grid() {
        let pls_grid = init_grid(3, 0);
        assert_eq!(pls_grid.pls, vec![-4, -3, -2, -4, -3, -1, -4, -2, 0]);
        let pls_grid = init_grid(3, 10);
        assert_eq!(pls_grid.pls, vec![-3, -2, -1, -3, -1, 0, -3, -1, 1]);
        let pls_grid = init_grid(4, 0);
        assert_eq!(
            pls_grid.pls,
            vec![-4, -3, -2, -1, -4, -3, -1, 0, -4, -2, 0, 1, -4, -2, 0, 2]
        );
    }

    #[test]
    fn test_compute_sub_squares() {
        let pls_grid = init_grid(3, 0);
        let squares_grid = compute_sub_squares(&pls_grid);

        assert_eq!(squares_grid.pls, vec![-23]);

        let pls_grid = init_grid(4, 0);
        let squares_grid = compute_sub_squares(&pls_grid);

        assert_eq!(squares_grid.pls, vec![-23, -11, -20, -5]);
    }

    #[test]
    fn test_get_max_square_18() {
        let side = 300;
        let serial_n = 18;
        let pls_grid = init_grid(side, serial_n);
        let squares_grid = compute_sub_squares(&pls_grid);
        assert_eq!(get_max_square(&squares_grid), (29, (33, 45)));
    }

    #[test]
    fn test_get_max_square_42() {
        let side = 300;
        let serial_n = 42;
        let pls_grid = init_grid(side, serial_n);
        let squares_grid = compute_sub_squares(&pls_grid);
        assert_eq!(get_max_square(&squares_grid), (30, (21, 61)));
    }

    #[test]
    fn test_compute_sub_squares_from_prev() {
        let grid = init_grid(3, 0);
        let two_grid = compute_sub_squares_from_prev(&grid, &grid);
        let three_grid = compute_sub_squares_from_prev(&grid, &two_grid);

        assert_eq!(three_grid.pls, vec![-23]);

        let grid = init_grid(4, 0);
        let two_grid = compute_sub_squares_from_prev(&grid, &grid);
        let three_grid = compute_sub_squares_from_prev(&grid, &two_grid);

        assert_eq!(three_grid.pls, vec![-23, -11, -20, -5]);
    }

    #[test]
    fn test_compute_sub_squares_from_prev_18() {
        let side = 300;
        let serial_n = 18;
        let grid = init_grid(side, serial_n);

        let two_grid = compute_sub_squares_from_prev(&grid, &grid);
        let three_grid = compute_sub_squares_from_prev(&grid, &two_grid);

        assert_eq!(get_max_square(&three_grid), (29, (33, 45)));
    }

    #[test]
    fn test_compute_sub_squares_from_prev_42() {
        let side = 300;
        let serial_n = 42;
        let grid = init_grid(side, serial_n);

        let two_grid = compute_sub_squares_from_prev(&grid, &grid);
        let three_grid = compute_sub_squares_from_prev(&grid, &two_grid);

        assert_eq!(get_max_square(&three_grid), (30, (21, 61)));
    }
}
