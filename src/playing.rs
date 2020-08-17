use super::gobblet::{Board, Player};
fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect()
}

pub fn play(p1: &mut Player, p2: &mut Player, first_ai: bool, second_ai: bool, board: &mut Board) {
    loop {
        let f: bool;
        let n_player: i32 = board.get_player();

        if n_player == 0 {
            f = first_ai;
        } else {
            f = second_ai;
        }

        println!("---Now Board---");
        println!("{}", board);
        println!("---------------");

        if f {
            println!("AI turn");

            match n_player {
                0 => board.ai_action(p1,p2),
                1 => board.ai_action(p2,p1),
                _ => (),
            }
        } else {
            println!("0 -> put 1 -> move");
            let x: i32 = read();

            if x == 0 {
                println!("which piece??");
                let piece: usize = read();
                println!("Where ?? 0 <= x <= 2, 0 <= y <= 2");
                let cord: Vec<usize> = read_vec();
                let query = (0, piece, cord[0], cord[1], 0, 0);

                match n_player {
                    0 => board.cope_query(&query, p1),
                    1 => board.cope_query(&query, p2),
                    _ => (),
                }
            } else {
                println!("Which ?? 0 <= x <= 2, 0 <= y <= 2");
                let old: Vec<usize> = read_vec();
                println!("To ?? 0 <= x <= 2, 0 <= y <= 2");
                let cord: Vec<usize> = read_vec();

                let piece = board.get_piece(old[0], old[1]);

                let query = (1, piece, old[0], old[1], cord[0], cord[1]);
                match n_player {
                    0 => board.cope_query(&query, p1),
                    1 => board.cope_query(&query, p2),
                    _ => (),
                }
            }
        }

        if board.check_win() {
            let x = board.get_player();
            println!("Player {} win. OK???", x + 1);
            break;
        }

        board.change_turn();
    }
}
