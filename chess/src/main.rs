fn main() {
    let mut board = Board::starting_pos();
    
    let test_fen = String::from("8/q4k2/8/4q3/2K5/8/8/4B3 w K - 0 1");

    board.read_fen(test_fen);
    board.print_board();
    board.print_fen();
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
                        for _ in 0..j.to_digit(10).unwrap() - 1{
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

    fn print_board(&self) {
        for rows in self.squares.clone().into_iter(){
            for columns in rows {
                print!("|");
                match columns {
                    Piece::Blank => print!("_"),
                    Piece::RookWhite => print!("{}", char::from_u32(0x265C).unwrap()),
                    Piece::RookBlack => print!("{}", char::from_u32(0x2656).unwrap()),
                    Piece::KnightWhite => print!("{}", char::from_u32(0x265E).unwrap()),
                    Piece::KnightBlack => print!("{}", char::from_u32(0x2658).unwrap()),
                    Piece::QueenWhite => print!("{}", char::from_u32(0x265B).unwrap()),
                    Piece::QueenBlack => print!("{}", char::from_u32(0x2655).unwrap()),
                    Piece::KingBlack => print!("{}", char::from_u32(0x2654).unwrap()),
                    Piece::KingWhite => print!("{}", char::from_u32(0x265A).unwrap()),
                    Piece::PawnBlack => print!("{}", char::from_u32(0x2659).unwrap()),
                    Piece::PawnWhite => print!("{}", char::from_u32(0x265F).unwrap()),
                    Piece::BishopWhite => print!("{}", char::from_u32(0x265D).unwrap()),
                    Piece::BishopBlack => print!("{}", char::from_u32(0x2657).unwrap()),
                    
                }
                
            }
            println!("|");
        }
    } 

    fn print_fen(&self) {
        for row in &self.squares {
            println!("/");
            for column in row {
                println!("{:?}",column)
            }
        };

        println!("{} {} - {} {}",&self.color_move, &self.caslting_rights, &self.half_moves, &self.full_moves)

    }
}
