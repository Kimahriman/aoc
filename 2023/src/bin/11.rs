fn main() {
    let contents = std::fs::read_to_string("inputs/11.txt").unwrap();

    let mut empty_rows: Vec<usize> = Vec::new();
    let mut empty_cols: Vec<usize> = Vec::new();

    let grid: Vec<Vec<bool>> = contents
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect();

    let mut galaxies: Vec<(usize, usize)> = Vec::new();
    for (i, row) in grid.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if *cell {
                galaxies.push((i, j));
            }
        }
    }

    for (i, row) in grid.iter().enumerate() {
        if row.iter().all(|c| !*c) {
            empty_rows.push(i);
        }
    }

    for col in 0..grid[0].len() {
        if (0..grid.len()).all(|row| !grid[row][col]) {
            empty_cols.push(col);
        }
    }

    for space in [1, 999999] {
        let mut sum = 0;
        for (index, first) in galaxies.iter().enumerate() {
            for second in galaxies.iter().skip(index + 1) {
                let min_row = usize::min(first.0, second.0);
                let max_row = usize::max(first.0, second.0);
                let min_col = usize::min(first.1, second.1);
                let max_col = usize::max(first.1, second.1);

                let empty_rows_crossed = (min_row..max_row)
                    .filter(|r| empty_rows.contains(r))
                    .count();
                let empty_cols_crossed = (min_col..max_col)
                    .filter(|c| empty_cols.contains(c))
                    .count();
                sum += max_row - min_row + empty_rows_crossed * space;
                sum += max_col - min_col + empty_cols_crossed * space;
            }
        }

        println!("{}", sum);
    }
}
