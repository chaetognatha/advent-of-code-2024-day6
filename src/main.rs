fn main() {
    let puzzle_input: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    let puzzle_grid: Vec<Vec<char>> = puzzle_input
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    for row in &puzzle_grid {
        println!("{:?}", row);
        println!();
}

    let mut watchman = WatchMan::new(0, 0, '^');
    let mut current_grid = puzzle_grid;
    let mut movement = (0, 0);
    loop {
        current_grid = update_grid(&current_grid, &mut watchman, &mut movement);
        for row in &current_grid {
            println!("{:?}", row);
        }
        println!();
        let next_row = (watchman.row as isize + movement.0) as usize;
        let next_col = (watchman.col as isize + movement.1) as usize;
        if next_row >= current_grid.len() || next_col >= current_grid[0].len() {
            println!("Watchman has left the grid");
            break;
        }
    }

}

fn update_grid(grid: &Vec<Vec<char>>, watchman: &mut WatchMan, movement: &mut (isize, isize)) -> Vec<Vec<char>> {
    // on every update, the rotating character will move one step in the direction it is facing
    // if it is facing a # it will turn right (90 degrees clockwise)
    // it will leave an X in its place
    let mut new_grid = grid.clone();
    let directions = ['^', '>', 'v', '<'];
    

    for (row_index, row) in grid.iter().enumerate() {
        for (col_index, cell) in row.iter().enumerate() {
            if directions.contains(cell) {
                new_grid[row_index][col_index] = 'X';
                watchman.row = row_index;
                watchman.col = col_index;
                *movement = match watchman.direction {
                    '^' => (-1, 0),
                    '>' => (0, 1),
                    'v' => (1, 0),
                    '<' => (0, -1),
                    _ => panic!("Invalid direction"),
                }
            }
        }
    }

    let new_row = (watchman.row as isize + movement.0) as usize;
    let new_col = (watchman.col as isize + movement.1) as usize;
    // check if out of bounds before accessing coordinates
    if new_row >= new_grid.len() || new_col >= new_grid[0].len() {
        return new_grid;
    }

    if new_grid[new_row][new_col] == '#' {
        watchman.rotate_right();
        // cannot move to a wall, so we don't update the movement, but we still need to rotate character
        new_grid[watchman.row][watchman.col] = watchman.direction;
    } else { new_grid[new_row][new_col] = watchman.direction;}
   
    new_grid
}

struct WatchMan {
    // this class represents the rotating character
    // it has a position, a direction and a character representing the rotation
    // it only rotates right using a method
   row: usize,
   col: usize,
   direction: char,
}

impl WatchMan {
    fn new(row: usize, col: usize, direction: char) -> WatchMan {
        WatchMan {
            row,
            col,
            direction,
        }
    }
    fn get_position(&self) -> (usize, usize) {
        (self.row, self.col)
    }
    fn rotate_right(&mut self) {
        // this method will rotate the character 90 degrees clockwise
        // it will change the direction of the character
        // ^ -> >
        // > -> v
        // v -> <
        // < -> ^
        match self.direction {
            '^' => self.direction = '>',
            '>' => self.direction = 'v',
            'v' => self.direction = '<',
            '<' => self.direction = '^',
            _ => panic!("Invalid direction"),
        }
    }
}