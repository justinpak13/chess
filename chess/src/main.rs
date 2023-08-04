fn main() {
    let mut board = Board::starting_pos();
    let test_fen = String::from("rnb1kbn1/ppp1ppp1/3q3B/3p3p/3P3P/5N1R/PPP1PPP1/RN1QKB2 b Qq - 2 5");
    board.read_fen(test_fen);
    board.print();
    //let black = vec![0x2654,0x2655, 0x2656, 0x2657, 0x2658, 0x2659];
    //let white = vec![0x265A,0x265B, 0x265C, 0x265D, 0x265E, 0x265F];
    //for j in white.iter(){
    //    println!("{:?}",char::from_u32(*j).unwrap());
    //}
    //for j in black.iter(){
    //    println!("{:?}",char::from_u32(*j).unwrap());
    //}
}

#[derive(Clone,Debug)]
enum Piece {
    Blank,
    KingBlack,
    QueenBlack,
    RookBlack,
    KnightBlack,
    BishopBlack,
    PawnBlack,
    KingWhite,
    QueenWhite,
    RookWhite,
    KnightWhite,
    BishopWhite,
    PawnWhite,
}

#[derive(Debug)]
struct Board {
    squares: Vec<Vec<Piece>>,
    color_move: char,
    caslting_rights: String,
    half_moves: i8,
    full_moves: i8

}

impl Board {
    fn _new() -> Board  {
        let result:Board = Board { squares: (vec![vec![Piece::Blank;8];8]), color_move: 'w', caslting_rights: String::from("KQkq"), half_moves: 0, full_moves: 0 } ;
        result
    }

    fn read_fen(&mut self, fen: String){

        let fen_split: Vec<&str> = fen.split(" ").collect();
        self.color_move = fen_split[1].parse().unwrap();
        self.caslting_rights = String::from(fen_split[2]);
        self.half_moves = fen_split[4].parse().unwrap();
        self.full_moves = fen_split[5].parse().unwrap();

        let coordinates: Vec<&str> = fen_split[0].split("/").collect();
        self.squares = vec![vec![Piece::Blank;8];8];

        for i in 0..8{
            let row = i;
            let mut counter: u32 = 0;

            for j in coordinates[row].chars().into_iter(){
                if counter > 8 {
                    break
                }

                match j {
                    'r' => self.squares[i][counter as usize] = Piece::RookBlack,
                    'n' => self.squares[i][counter as usize] = Piece::KnightBlack,
                    'b' => self.squares[i][counter as usize] = Piece::BishopBlack,
                    'q' => self.squares[i][counter as usize] = Piece::QueenBlack,
                    'k' => self.squares[i][counter as usize] = Piece::KingBlack,
                    'p' => self.squares[i][counter as usize] = Piece::PawnBlack,
                    'R' => self.squares[i][counter as usize] = Piece::RookWhite,
                    'N' => self.squares[i][counter as usize] = Piece::KnightWhite,
                    'B' => self.squares[i][counter as usize] = Piece::BishopWhite,
                    'Q' => self.squares[i][counter as usize] = Piece::QueenWhite,
                    'K' => self.squares[i][counter as usize] = Piece::KingWhite,
                    'P' => self.squares[i][counter as usize] = Piece::PawnWhite,
                    _ => {
                        for count in 0..j.to_digit(10).unwrap() - 1{
                            self.squares[i][counter as usize] = Piece::Blank;
                            counter += 1;
                        }
                    }
                    
                    
                }
                counter += 1;

            }

        }



    }

    fn starting_pos() -> Board {
        let starting_fen = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        let mut result = Board::_new();

        result.read_fen(starting_fen);

        result
    }

    fn print(&self) {
        for rows in self.squares.clone().into_iter(){
            for columns in rows {
                match columns {
                    Piece::Blank => print!("_"),
                    Piece::RookWhite => print!("R"),
                    Piece::RookBlack => print!("r"),
                    Piece::KnightWhite => print!("N"),
                    Piece::KnightBlack => print!("n"),
                    Piece::QueenWhite => print!("Q"),
                    Piece::QueenBlack => print!("q"),
                    Piece::KingBlack => print!("k"),
                    Piece::KingWhite => print!("K"),
                    Piece::PawnBlack => print!("p"),
                    Piece::PawnWhite => print!("P"),
                    Piece::BishopWhite => print!("B"),
                    Piece::BishopBlack => print!("b"),
                    
                }
                
            }
            println!("");
        }
    } 
}
