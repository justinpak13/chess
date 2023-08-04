fn main() {
    let black = vec![0x2654,0x2655, 0x2656, 0x2657, 0x2658, 0x2659];
    let white = vec![0x265A,0x265B, 0x265C, 0x265D, 0x265E, 0x265F];
    for j in white.iter(){
        println!("{:?}",char::from_u32(*j).unwrap());
    }
    for j in black.iter(){
        println!("{:?}",char::from_u32(*j).unwrap());
    }
}

#[derive(Clone,Debug)]
enum Piece {
    Blank,
    King,
    Queen,
    Rook,
    Knight,
    Bishop,
    Pawn
}

#[derive(Debug)]
struct Board {
    board: Vec<Vec<Piece>>,
}

impl Board{
    fn new() -> Board {

    }

}

