use std::fmt;

#[derive(Debug, Clone)]
pub struct Board {
    board: Vec<Vec<Vec<i32>>>,
    appeard_board: Vec<Vec<i32>>,
    player: i32,
    depth : i32,
    sudden_win : bool,
}

#[derive(Debug, Clone)]
pub struct Player {
    piece: Vec<i32>,
}

impl Board {
    pub fn new(first_player: i32, ai_depth : i32) -> Self {
        // let appeard_board : Vec<Vec<i32>> = vec![vec![0,0,0],vec![0,3,0],vec![-3,0,-3]];
        // let mut board = vec![vec![vec![0; 3]; 3]; 3];
        // for i in 0..3{
        //     for j in 0..3{
        //         if appeard_board[i][j].abs() > 0{
        //             let sign = appeard_board[i][j].signum();
        //             let x = appeard_board[i][j].abs() as usize;

        //             board[i][j][x-1] = 1 * sign;
        //         }
        //     }
        // }
        Board {
            board: vec![vec![vec![0; 3]; 3]; 3],
            appeard_board: vec![vec![0; 3]; 3],
            // board: board,
            // appeard_board: appeard_board,
            player: first_player,
            depth : ai_depth,
            sudden_win : false,
        }
    }

    pub fn get_player(&self) -> i32 {
        self.player
    }

    pub fn change_turn(&mut self) {
        self.player = 1 - self.player;
    }

    pub fn get_piece(&self, x: usize, y: usize) -> usize {
        self.appeard_board[x][y].abs() as usize
    }

    pub fn is_put(&self, x: usize, y: usize, k: usize) -> bool {
        if x > 2 || y > 2 {
            return false;
        }

        (self.appeard_board[x][y].abs() as usize) < k
    }
    pub fn is_move(&self, x: usize, y: usize, k: usize, nx: usize, ny: usize) -> bool {
        if x > 2 || y > 2 || nx > 2 || ny > 2{
            return false;
        }

        (self.appeard_board[nx][ny].abs() as usize) < k && self.appeard_board[x][y].signum() == (-2*self.player + 1)
    }

    // pub fn is_move_for_ai(&mut self, x: usize, y: usize, k: usize, nx: usize, ny: usize) -> bool {
    //     if x > 2 || y > 2 || nx > 2 || ny > 2{
    //         return false;
    //     }

    //     if (self.appeard_board[nx][ny].abs() as usize) >= k || self.appeard_board[x][y].signum() != (-2*self.player + 1){
    //         return false;
    //     }

    //     self.board[x][y][k - 1] = 0;
    //     self.appeard_board[x][y] = 0;

    //     for i in 0..3 {
    //         if self.board[x][y][i].abs() > 0 {
    //             self.appeard_board[x][y] = ((i + 1) as i32) * self.board[x][y][i].signum();
    //         }
    //     }

    //     if self.check_win(){
    //         self.sudden_win = true;
    //     }

    //     self.board[x][y][k - 1] = 1 * (-2*self.player + 1);
    //     self.appeard_board[x][y] = 0;

    //     for i in 0..3 {
    //         if self.board[x][y][i].abs() > 0 {
    //             self.appeard_board[x][y] = ((i + 1) as i32) * self.board[x][y][i].signum();
    //         }
    //     }

    //     !self.sudden_win
    // }

    pub fn put(&mut self, x: usize, y: usize, k: usize) -> Result<(), &'static str> {
        if !self.is_put(x, y, k) {
            let s_err = format!("Unexpected input(x,y) = ({},{}) , piece = {}", x, y, k);
            println!("{}", s_err);
            return Err("remove error");
        }
        self.board[x][y][k - 1] = 1 * (-2 * self.player + 1);
        let k = k as i32;
        self.appeard_board[x][y] = k * (-2 * self.player + 1);
        Ok(())
    }

    fn remove(&mut self, x: usize, y: usize, k: usize) -> Result<(), &'static str> {
        if self.appeard_board[x][y] != (k as i32) * (-2 * self.player + 1) {
            println!("{}", self);
            let s_err = format!("Unexpected input(x,y) = ({},{}) , piece = {}", x, y, k);
            println!("{}", s_err);
            return Err("remove error");
        }
        self.board[x][y][k - 1] = 0;
        self.appeard_board[x][y] = 0;
        for i in 0..3 {
            if self.board[x][y][i].abs() > 0 {
                self.appeard_board[x][y] = ((i + 1) as i32) * self.board[x][y][i].signum();
            }
        }
        Ok(())
    }

    fn move_piece(
        &mut self,
        x: usize,
        y: usize,
        k: usize,
        nx: usize,
        ny: usize,
    ) -> Result<(), &'static str> {
        if !self.is_move(x,y,k,nx,ny) {
            let s_err = format!("Unexpected input(x,y) = ({},{}) , piece = {}", nx, ny, k);
            println!("{}",self);
            println!("{}", s_err);
            return Err("move error");
        }
        
        self.board[x][y][k - 1] = 0;
        self.appeard_board[x][y] = 0;

        for i in 0..3 {
            if self.board[x][y][i].abs() > 0 {
                self.appeard_board[x][y] = ((i + 1) as i32) * self.board[x][y][i].signum();
            }
        }

        // if self.check_win(){
        //     self.sudden_win = true;
        // }

        self.board[nx][ny][k - 1] = 1 * (-2 * self.player + 1);
        self.appeard_board[nx][ny] = (k as i32) * (-2 * self.player + 1);
        Ok(())
    }

    pub fn check_win(&mut self) -> bool {
        let mut win_flag = false | self.sudden_win;

        if self.sudden_win{
            self.change_turn();
        }

        let judge = { |x, y, z| (x > 0 && y > 0 && z > 0) || (x < 0 && y < 0 && z < 0) };

        for i in 0..3 {
            win_flag |= judge(
                self.appeard_board[i][0],
                self.appeard_board[i][1],
                self.appeard_board[i][2],
            );
            win_flag |= judge(
                self.appeard_board[0][i],
                self.appeard_board[1][i],
                self.appeard_board[2][i],
            );
        }

        win_flag |= judge(
            self.appeard_board[0][0],
            self.appeard_board[1][1],
            self.appeard_board[2][2],
        );
        win_flag |= judge(
            self.appeard_board[0][2],
            self.appeard_board[1][1],
            self.appeard_board[2][0],
        );

        win_flag
    }

    pub fn check_reach(&self,first:bool) -> i32 {
        let mut cnt : i32= 0;
        let judge = {
            |x, y, z| {
                if !first {
                    (x < 0 && y < 0 && z == 0)
                    || (x < 0 && y == 0 && z < 0)
                    || (x == 0 && y < 0 && z < 0)
                }else{
                    (x > 0 && y > 0 && z == 0)
                    || (x > 0 && y == 0 && z > 0)
                    || (x == 0 && y < 0 && z > 0)
                }    
            }
        };

        for i in 0..3 {
            if judge(
                self.appeard_board[i][0],
                self.appeard_board[i][1],
                self.appeard_board[i][2],
            ) {
                cnt += 1
            }
            if judge(
                self.appeard_board[0][i],
                self.appeard_board[1][i],
                self.appeard_board[2][i],
            ) {
                cnt += 1
            }
        }

        if judge(
            self.appeard_board[0][0],
            self.appeard_board[1][1],
            self.appeard_board[2][2],
        ) {
            cnt += 1
        }
        if judge(
            self.appeard_board[0][2],
            self.appeard_board[1][1],
            self.appeard_board[2][0],
        ) {
            cnt += 1
        }

        cnt
    }

    pub fn cope_query(&mut self, q: &(usize, usize, usize, usize, usize, usize), p: &mut Player) {
        if q.0 == 0 {
            self.put(q.2, q.3, q.1).expect("cope put error");
            p.piece[q.1 - 1] -= 1;
        } else {
            self.move_piece(q.2, q.3, q.1, q.4, q.5)
                .expect("cope move error");
        }
        // println!("Done query : {:?}", q);
        // println!("{}",self);
        // let mut s = String::new();
        // std::io::stdin().read_line(&mut s).ok();
    }

    fn undo_query(&mut self, q: &(usize, usize, usize, usize, usize, usize), p: &mut Player) {
        // println!("{:?}",q);
        if q.0 == 0 {
            self.remove(q.2, q.3, q.1).expect("undo put error");
            p.piece[q.1 - 1] += 1;
        } else {
            self.move_piece(q.4, q.5, q.1, q.2, q.3)
                .expect("undo move error");
        }
        // println!("Undone query : {:?}", q);
        // println!("{}",self);
        // let mut s = String::new();
        // std::io::stdin().read_line(&mut s).ok();
    }

    fn alpha_beta(
        &mut self,
        is_ai: bool,
        alpha: i32,
        beta: i32,
        level: i32,
        p_ai: &mut Player,
        p_op: &mut Player,
    ) -> i32 {
        if level == 0 || self.check_win() {
            return self.evaluate(is_ai,p_ai);
        }

        let mut n_alpha = -100000000;
        let mut n_beta = 100000000;

        let hand = get_all_hand(p_ai, self, !is_ai);
        let mut best_query: (usize, usize, usize, usize, usize, usize) = (0,0,0,0,0,0);

        // if level == self.depth - 2{
        //     println!("{}",self);
        //     println!("{:?}",hand);
        //     println!("The first kanousei {}",hand.len());
        // }
       
        if is_ai {
            for e in &hand {
                self.cope_query(e, p_ai);
                self.change_turn();
                let child_score = self.alpha_beta(!is_ai, alpha, beta, level - 1, p_ai,p_op);
                self.change_turn();
                self.undo_query(e, p_ai);

                // println!("child_score : {}",child_score);

                if child_score > n_alpha {
                    if level == self.depth{
                        println!("Update done child_score : {}", child_score);
                    }
                    best_query = *e;
                    n_alpha = child_score;
                }

                if n_alpha >= beta {
                    break;
                }
            }

            if level == self.depth{
                println!("AI query {:?}", &best_query);
                self.cope_query(&best_query, p_ai);
            }

            return n_alpha;
        } else {
            for e in &hand {
                self.cope_query(e, p_op);
                self.change_turn();
                let child_score = self.alpha_beta(!is_ai, alpha, beta, level - 1, p_ai,p_op);
                self.change_turn();
                self.undo_query(e, p_op);

                if n_beta > child_score {
                    n_beta = child_score;
                }
                if n_beta <= alpha {
                    break;
                }
            }

            return n_beta;
        }
    }

    fn evaluate(&mut self, flag: bool,p: &Player) -> i32 {
        let is_win = self.check_win();
        if is_win && !flag {
            return 200;
        } else if is_win && flag {
            return -200;
        } else {
            // if flag {
            //     return 100 - level;
            // } else {
            //     return level - 100;
            // }
            let f_reach = self.check_reach(true);
            let s_reach = self.check_reach(false);

            let hand_score = p.piece[0] * 1 + p.piece[1]* 2 + p.piece[2]*3;
            let mut board_score = 0;

            for i in 0..3{
                for j in 0..3{
                    if self.appeard_board[i][j] > 0 && (i+j)%2 == 0{
                        board_score += 1;
                    }
                }
            }
            
            return (s_reach - f_reach) * 20 +  hand_score + board_score;
        }
    }

    pub fn ai_action(&mut self, p_ai: &mut Player,p_op: &mut Player) {
        println!("Start");
        let score = self.alpha_beta(true, -100000000, 10000000, self.depth, p_ai,p_op);
        println!("AI evaluation : {}", score);
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for e in &self.appeard_board {
            for ee in e {
                write!(f, "{} ", ee)?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

impl Player {
    pub fn new() -> Self {
        Player {
            piece: vec![2, 2, 2],
        }
    }

    pub fn chenge(&mut self, x : Vec<i32>){
        self.piece = x;
    }
}

fn get_all_hand(
    p: &Player,
    b: &mut Board,
    first: bool,
) -> Vec<(usize, usize, usize, usize, usize, usize)> {
    // return tuple
    // 0 -> put 1 -> move
    // query x y dx dy
    let mut vec: Vec<(usize, usize, usize, usize, usize, usize)> = Vec::new();
    // for (i, piece) in p.piece.iter().enumerate() {
    //     if *piece == 0 {
    //         continue;
    //     }

    //     for j in 0..3 {
    //         for k in 0..3 {
    //             if b.is_put(j, k, i + 1) {
    //                 vec.push((0, i + 1, j, k, 0, 0));
    //             }
    //         }
    //     }
    // }

    for i in 0..3 {
        for j in 0..3 {
            if (first && b.appeard_board[i][j] > 0) || (!first && b.appeard_board[i][j] < 0) {
                let dx: Vec<i32> = vec![1, 1, 0, -1, -1, -1, 0, 1];
                let dy: Vec<i32> = vec![0, 1, 1, 1, 0, -1, -1, -1];

                for kk in 0..8 {
                    let x = i as i32;
                    let y = j as i32;
                    let k = b.appeard_board[i][j].abs() as usize;

                    let nx = x + dx[kk];
                    let ny = y + dy[kk];

                    if nx < 0 || ny < 0 {
                        continue;
                    }

                    let nx = nx as usize;
                    let ny = ny as usize;

                    // if x == 1 && k == 3 && i == 1 && j == 1 && nx == 1 && ny == 0{
                    //     println!("{}",b.is_move(i, j, k, nx, ny));
                    // }

                    if b.is_move(i,j,k,nx,ny) {
                        vec.push((1, k, i, j, nx, ny));
                    }
                }
            }
        }
    }
    for (i, piece) in p.piece.iter().enumerate() {
        if *piece == 0 {
            continue;
        }

        for j in 0..3 {
            for k in 0..3 {
                if b.is_put(j, k, i + 1) {
                    vec.push((0, i + 1, j, k, 0, 0));
                }
            }
        }
    }

    vec
}
