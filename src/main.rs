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
}
}

fn update_grid(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_grid = grid.clone();

    for (row_index, row) in grid.iter().enumerate() {
        for (col_index, cell) in row.iter().enumerate() {
            if cell == &'^' {
                new_grid[row_index][col_index] = 'X';
            }
        }
    }

    new_grid
}

fn check_direction(grid: &Vec<Vec<char>>, row: usize, col: usize) -> char {
    let directions = [ 
        ('^', -1, 0), 
        ('v', 1, 0), 
        ('<', 0, -1), 
        ('>', 0, 1) 
    ];

    for &(direction, row_offset, col_offset) in &directions {
        let new_row = row as isize + row_offset;
        let new_col = col as isize + col_offset;

        if new_row < 0 || new_row >= grid.len() as isize || new_col < 0 || new_col >= grid[0].len() as isize {
            continue;
        }

        if grid[new_row as usize][new_col as usize] == '#' {
            return direction;
        }
    }

    '^'
}