use std::io;

fn draw_game(game: &[[char; 3]; 3]) {
    for row in game.iter() {
        println!("{} | {} | {}", row[0], row[1], row[2]);
    }
}

fn get_cmd(game: &mut [[char; 3]; 3], turn: &mut i32) {
    loop {
        println!("Enter row (0-2):");
        let mut row_input = String::new();
        io::stdin()
            .read_line(&mut row_input)
            .expect("Failed to read line");
        let row: usize = match row_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number between 0 and 2.");
                continue;
            }
        };

        println!("Enter column (0-2):");
        let mut col_input = String::new();
        io::stdin()
            .read_line(&mut col_input)
            .expect("Failed to read line");
        let col: usize = match col_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number between 0 and 2.");
                continue;
            }
        };

        if game[row][col] == ' ' {
            game[row][col] = 'X';
            *turn = 0;
            break;
        } else {
            println!("Cell already occupied. Try again.");
        }
    }

    draw_game(&game);
}

fn value(game: &[[char; 3]; 3]) -> i32 {
    if terminal(game, 'O') {
        return 1;
    } else if terminal(game, 'X') {
        return -1;
    } else if is_draw(game) {
        return 0;
    }

    0
}

fn ai(turn: &mut i32, game: &mut [[char; 3]; 3]) -> i32 {
    if terminal(&game, 'O') || terminal(&game, 'X') || is_draw(&game) {
        return value(&game);
    }

    let mut best_score = i32::MIN;
    let mut best_move = (0, 0);

    for row in 0..3 {
        for col in 0..3 {
            if game[row][col] == ' ' {
                game[row][col] = 'O';
                let score = minimax(game, false);
                game[row][col] = ' '; 

                if score > best_score {
                    best_score = score;
                    best_move = (row, col);
                }
            }
        }
    }

    game[best_move.0][best_move.1] = 'O';
    *turn = 1;

    draw_game(&game);

    best_score
}

fn minimax(game: &mut [[char; 3]; 3], maximizing: bool) -> i32 {
    if terminal(&game, 'O') || terminal(&game, 'X') || is_draw(&game) {
        return value(&game);
    }

    if maximizing {
        let mut best_score = i32::MIN;
        for row in 0..3 {
            for col in 0..3 {
                if game[row][col] == ' ' {
                    game[row][col] = 'O';
                    let score = minimax(game, false);
                    game[row][col] = ' '; 
                    best_score = best_score.max(score);
                }
            }
        }
        best_score
    } else {
        let mut best_score = i32::MAX;
        for row in 0..3 {
            for col in 0..3 {
                if game[row][col] == ' ' {
                    game[row][col] = 'X';
                    let score = minimax(game, true);
                    game[row][col] = ' '; 
                    best_score = best_score.min(score);
                }
            }
        }
        best_score
    }
}

fn terminal(game: &[[char; 3]; 3], player: char) -> bool {
    for i in 0..3 {
        if game[i][0] == player && game[i][1] == player && game[i][2] == player {
            return true;
        }
        if game[0][i] == player && game[1][i] == player && game[2][i] == player {
            return true;
        }
    }

    if game[0][0] == player && game[1][1] == player && game[2][2] == player {
        return true;
    }
    if game[0][2] == player && game[1][1] == player && game[2][0] == player {
        return true;
    }

    false
}

fn is_draw(game: &[[char; 3]; 3]) -> bool {
    for row in game.iter() {
        for &cell in row.iter() {
            if cell == ' ' {
                return false;
            }
        }
    }
    true
}

fn main() {
    println!("WELCOME TO TIC_TAC_TOE");
    let mut game: [[char; 3]; 3] = [[' '; 3]; 3];
    let mut turn: i32 = 1;
    draw_game(&game);
    loop {
        if turn == 1 {
            get_cmd(&mut game, &mut turn);
            if terminal(&game, 'X') {
                println!("You won!");
                break;
            } else if is_draw(&game) {
                println!("It's a draw!");
                break;
            }
        } else {
            ai(&mut turn, &mut game);
            if terminal(&game, 'O') {
                println!("AI won!");
                break;
            } else if is_draw(&game) {
                println!("It's a draw!");
                break;
            }
        }
    }
}
