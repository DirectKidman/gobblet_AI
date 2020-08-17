mod gobblet;
mod playing;

use crate::gobblet::{Board, Player};

fn main() {
    let mut b = Board::new(0,6);
    let mut p1 = Player::new();
    let mut p2 = Player::new();

    // p1.chenge(vec![2,1,1]);
    // p2.chenge(vec![2,2,0]);

    println!("{}", &b);
    println!("{:?}", &p1);
    println!("{:?}", &p2);

    // playing::play(&mut p1, &mut p2, false, false, &mut b);
    playing::play(&mut p1, &mut p2, false, true, &mut b);

    println!("{}", &b);
}
