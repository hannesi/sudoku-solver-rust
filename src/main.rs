use std::io;
use std::time::SystemTime;

type Sudoku = [[u8; 9]; 9];

fn main() {
    println!("====Sudoku Solver====\n");
    let mut sudoku = load_sudoku();
    println!("Input:");
    show(&sudoku);
    let start_time = SystemTime::now();
    let solved = solve(&mut sudoku);
    let duration: u128 = match start_time.elapsed() {
        Ok(elapsed) => {
            elapsed.as_millis()
        }
        Err(e) => {
            println!("Error happened while timing the solving process: {}", e);
            0
        }
    };
    if solved {
        println!("Solution:");
        show(&sudoku);
        println!("Solved in {} seconds!", (duration as f64) / 1000 as f64);
    } else {
        println!{"Couldn't find a solution for the sudoku :("};
    }
}

fn load_sudoku() -> Sudoku {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(io::stdin());
    let mut sudoku: Sudoku = [[0 as u8; 9] ; 9];
    let mut row_no: usize = 0;
    for result in reader.records() {
        match result {
            Ok(r) => {
                let mut col_no: usize = 0;
                for character in r.as_slice().chars() {
                    sudoku[row_no][col_no] = character as u8 - 48;
                    col_no += 1;
                }
                row_no += 1;
            },
            Err(e) => println!("{}", e)
        }
    }
    sudoku
}

fn show(sudoku: &Sudoku) {
    for row in 0..sudoku.len() {
        for col in 0..sudoku[row].len() {
            print!("{} ", sudoku[row][col]);
            if (col + 1) % 3 == 0 && col < 8 {
                print!("| ");
            }
        }
        println!("");
        if (row + 1) % 3 == 0 && row < 8 {
            println!("---------------------");
        } 
    }
    println!();
}

fn solve(sudoku: &mut Sudoku) -> bool {
    solve_cell(sudoku, 0, 0)
}

fn solve_cell(sudoku: &mut Sudoku, row: usize, col: usize) -> bool {
    // if on 9th row, a solution has been found
    if row == 9 {
        return true
    }

    // next cells coordinates
    let next_col: usize = (col + 1) % 9;
    let next_row: usize = if next_col >= col { row } else { row + 1 };

    // if the value of the current cell isn't 0, it's not getting edited
    if sudoku[row][col] != 0 {
        return solve_cell(sudoku, next_row, next_col)
    } 

    // top-left cell of the 3x3 area current cell is part of
    let area_col = col - col % 3;
    let area_row = row - row % 3;

    'candidate: for candidate_number in 1..10 {
        // println!("Tring to fit {} on [{}, {}]", candidate_number, row, col);
        // check row for candidate
        for col_no in 0..9 {
            if sudoku[row][col_no] == candidate_number {
                continue 'candidate
            }
        }
        // check col for candidate
        for row_no in 0..9 {
            if sudoku[row_no][col] == candidate_number {
                continue 'candidate
            }
        }
        // check 3x3 area for candidate
        for j in 0..3 {
            for k in 0..3 {
                if sudoku[j + area_row][k + area_col] == candidate_number {
                    continue 'candidate
                }
            }
        }
        
        // if all checks pass, set candidate_number as cell's value
        sudoku[row][col] = candidate_number;

        // dig deeper with the newfound value
        let solved = solve_cell(sudoku, next_row, next_col);
        // if we hit row "9" at a deeper stage, return the good news!
        if solved {
            return solved
        }
    }
    // if no candidate fits, set cell's value back to zero and let the previous level loop forwards
    sudoku[row][col] = 0;
    return false
}
