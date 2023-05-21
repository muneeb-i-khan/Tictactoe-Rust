use std::io;

fn draw_board(arr: [[char; 3]; 3]) {
    println!("   A       B       C");
    for row in &arr {
        for element in row {
            print!("   {}   |", element);
        }
        println!("\n");
    }
}

fn get_cmd(arr: &mut [[char; 3]; 3], usr: &mut bool) {
    if *usr == true {
        println!("X's turn");
    } else {
        println!("0's turn");
    }
    let mut cmd: String = String::new();
    io::stdin()
        .read_line(&mut cmd)
        .expect("Failed to read input");
    cmd = cmd.trim().to_string();
    if *usr == true {
        if cmd == "00" {
            if arr[0][0] != ' ' {
                *usr = false;
                println!("Incorrect input");
            } else {
                arr[0][0] = 'X';
            }
        } else if cmd == "01" {
            if arr[0][1] != ' ' {
                *usr = false;
                println!("Incorrect input");
            } else {
                arr[0][1] = 'X';
            }
        } else if cmd == "02" {
            if arr[0][2] != ' ' {
                *usr = false;
                println!("Incorrect input");
            } else {
                arr[0][2] = 'X';
            }
        } else if cmd == "10" {
            if arr[1][0] != ' ' {
                *usr = false;
                println!("Incorrect input");
            } else {
                arr[1][0] = 'X';
            }
        } else if cmd == "11" {
            if arr[1][1] != ' ' {
                *usr = false;
                println!("Incorrect input");
            } else {
                arr[1][1] = 'X';
            }
        } else if cmd == "12" {
            if arr[1][2] != ' ' {
                *usr = false;
                println!("Incorrect input");
            } else {
                arr[1][2] = 'X';
            }
        } else if cmd == "20" {
            if arr[2][0] != ' ' {
                *usr = false;
                println!("Incorrect input");
            } else {
                arr[2][0] = 'X';
            }
        } else if cmd == "21" {
            if arr[2][1] != ' ' {
                *usr = false;
                println!("Incorrect input");
            } else {
                arr[2][1] = 'X';
            }
        } else if cmd == "22" {
            if arr[2][2] != ' ' {
                *usr = false;
                println!("Incorrect input");
            } else {
                arr[2][2] = 'X';
            }
        } else {
            *usr = false;
            println!("Incorrect input");
        }
    } else {
        if cmd == "00" {
            if arr[0][0] != ' ' {
                *usr = true;
                println!("Incorrect input");
            } else {
                arr[0][0] = '0';
            }
        } else if cmd == "01" {
            if arr[0][1] != ' ' {
                *usr = true;
                println!("Incorrect input");
            } else {
                arr[0][1] = '0';
            }
        } else if cmd == "02" {
            if arr[0][2] != ' ' {
                *usr = true;
                println!("Incorrect input");
            } else {
                arr[0][2] = '0';
            }
        } else if cmd == "10" {
            if arr[1][0] != ' ' {
                *usr = true;
                println!("Incorrect input");
            } else {
                arr[1][0] = '0';
            }
        } else if cmd == "11" {
            if arr[1][1] != ' ' {
                *usr = true;
                println!("Incorrect input");
            } else {
                arr[1][1] = '0';
            }
        } else if cmd == "12" {
            if arr[1][2] != ' ' {
                *usr = true;
                println!("Incorrect input");
            } else {
                arr[1][2] = '0';
            }
        } else if cmd == "20" {
            if arr[2][0] != ' ' {
                *usr = true;
                println!("Incorrect input");
            } else {
                arr[2][0] = '0';
            }
        } else if cmd == "21" {
            if arr[2][1] != ' ' {
                *usr = true;
                println!("Incorrect input");
            } else {
                arr[2][1] = '0';
            }
        } else if cmd == "22" {
            if arr[2][2] != ' ' {
                *usr = true;
                println!("Incorrect input");
            } else {
                arr[2][2] = '0';
            }
        } else {
            *usr = true;
            println!("Incorrect input");
        }
    }
    draw_board(*arr);
}

fn chk_win(arr: &mut [[char; 3]; 3], win: &mut bool) {
    if arr[0][0] == 'X' && arr[0][1] == 'X' && arr[0][2] == 'X' {
        println!("X won");
        *win = true;
    } else if arr[1][0] == 'X' && arr[1][1] == 'X' && arr[1][2] == 'X' {
        println!("X won");
        *win = true;
    } else if arr[2][0] == 'X' && arr[2][1] == 'X' && arr[2][2] == 'X' {
        println!("X won");
        *win = true;
    } else if arr[0][0] == 'X' && arr[1][1] == 'X' && arr[2][2] == 'X' {
        println!("X won");
        *win = true;
    } else if arr[0][2] == 'X' && arr[1][1] == 'X' && arr[2][0] == 'X' {
        println!("X won");
        *win = true;
    } else if arr[0][0] == 'X' && arr[1][0] == 'X' && arr[2][0] == 'X' {
        println!("X won");
        *win = true;
    } else if arr[0][1] == 'X' && arr[1][1] == 'X' && arr[2][1] == 'X' {
        println!("X won");
        *win = true;
    } else if arr[0][2] == 'X' && arr[1][2] == 'X' && arr[2][2] == 'X' {
        println!("X won");
        *win = true;
    }

    if arr[0][0] == '0' && arr[0][1] == '0' && arr[0][2] == '0' {
        println!("0 won");
        *win = true;
    } else if arr[1][0] == '0' && arr[1][1] == '0' && arr[1][2] == '0' {
        println!("0 won");
        *win = true;
    } else if arr[2][0] == '0' && arr[2][1] == '0' && arr[2][2] == '0' {
        println!("0 won");
        *win = true;
    } else if arr[0][0] == '0' && arr[1][1] == '0' && arr[2][2] == '0' {
        println!("0 won");
        *win = true;
    } else if arr[0][2] == '0' && arr[1][1] == '0' && arr[2][0] == '0' {
        println!("0 won");
        *win = true;
    } else if arr[0][0] == '0' && arr[1][0] == '0' && arr[2][0] == '0' {
        println!("0 won");
        *win = true;
    } else if arr[0][1] == '0' && arr[1][1] == '0' && arr[2][1] == '0' {
        println!("0 won");
        *win = true;
    } else if arr[0][2] == '0' && arr[1][2] == '0' && arr[2][2] == '0' {
        println!("0 won");
        *win = true;
    }
}

fn main() {
    println!("Welcome to tictactoe game!");
    println!("Choose one of the following inputs:- ");
    println!("00 01 02");
    println!("10 11 12");
    println!("20 21 22");
    let mut game: [[char; 3]; 3] = [[' ', ' ', ' '], [' ', ' ', ' '], [' ', ' ', ' ']];
    let mut user: bool = true;
    let mut end: bool = false;

    draw_board(game);
    loop {
        get_cmd(&mut game, &mut user);
        user = !user;
        chk_win(&mut game, &mut end);
        if end == true {
            break;
        }
    }
    println!("GAME OVER!");
}
