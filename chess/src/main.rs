fn main() {
    let mut board = Board::starting_pos();
    
    let test_fen = String::from("8/8/8/8/8/8/8/Q7 w - - 0 20");

    board.read_fen(test_fen);
    board.print_board();

    get_legal_moves(board)
}

fn get_legal_moves(board: Board){
    for i in board.squares {
        println!("{:?}",i)
    }
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
    full_moves: i8,
    fen: Option<String>

}

impl Board {
    fn _new() -> Board  {
        let result:Board = Board { squares: (vec![vec![Piece::Blank;8];8]), color_move: 'w', caslting_rights: String::from("KQkq"), half_moves: 0, full_moves: 0, fen: None } ;
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

        self.fen = Some(get_fen(self));


    }

    fn starting_pos() -> Board {
        let starting_fen = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        let mut result = Board::_new();

        result.read_fen(starting_fen);
        result.fen = Some(get_fen(&result));

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

}


fn get_fen(board: &Board) -> String {
    let mut result = String::new();
    
    let mut counter = 0;
    for row in &board.squares {
        for column in row {
            match column {
                Piece::Blank => {
                    counter += 1;
                },
                Piece::RookWhite => {
                    if counter > 0 {
                        result.push_str(&counter.to_string());
                        counter = 0;
                    }
                    result.push('R');
                },
                Piece::RookBlack => {
                    if counter > 0 {
                        result.push_str(&counter.to_string());
                        counter = 0;
                    }
                    result.push('r')
                },
                Piece::KnightWhite => {
                    if counter > 0 {
                        result.push_str(&counter.to_string());
                        counter = 0;
                    }
                    result.push('N')
                },
                Piece::KnightBlack => {
                    if counter > 0 {
                        result.push_str(&counter.to_string());
                        counter = 0;
                    }
                    result.push('n')
                },
                Piece::QueenWhite => {
                    if counter > 0 {
                        result.push_str(&counter.to_string());
                        counter = 0;
                    }
                    result.push('Q')
                },
                Piece::QueenBlack => {
                    if counter > 0 {
                        result.push_str(&counter.to_string());
                        counter = 0;
                    }
                    result.push('q')
                },
                Piece::KingBlack => {
                    if counter > 0 {
                        result.push_str(&counter.to_string());
                        counter = 0;
                    }
                    result.push('k')
                },
                Piece::KingWhite => {
                    if counter > 0 {
                        result.push_str(&counter.to_string());
                        counter = 0;
                    }
                    result.push('K')
                },
                Piece::PawnBlack => {
                    if counter > 0 {
                        result.push_str(&counter.to_string());
                        counter = 0;
                    }
                    result.push('p')
                },
                Piece::PawnWhite => {
                    if counter > 0 {
                        result.push_str(&counter.to_string());
                        counter = 0;
                    }
                    result.push('P')
                },
                Piece::BishopWhite => {
                    if counter > 0 {
                        result.push_str(&counter.to_string());
                        counter = 0;
                    }
                    result.push('B')
                },
                Piece::BishopBlack => {
                    if counter > 0 {
                        result.push_str(&counter.to_string());
                        counter = 0;
                    }
                    result.push('b')
                },
                
            }
        }
        if counter > 0 {
            result.push_str(&counter.to_string());
            counter = 0;
        }
        result.push('/');
    };

    result.pop();
    result.push(' ');
    result.push(board.color_move);
    result.push(' ');
    result.push_str(&board.caslting_rights);
    result.push_str(" - ");
    result.push_str(&board.half_moves.to_string());
    result.push_str(" ");
    result.push_str(&board.full_moves.to_string());

    result

}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fen_0() {
        let test_board = Board::starting_pos();
        assert_eq!(test_board.fen.unwrap(), String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"))
    }

    #[test]
    fn test_fen_1() {
        let mut test_board = Board::starting_pos();
        test_board.read_fen("rnb1kbnr/pp1qpp1p/8/1BpP2p1/8/2N2N2/PPPP1PPP/R1BQK2R b KQkq - 0 5".to_string());
        assert_eq!(test_board.fen.unwrap(), String::from("rnb1kbnr/pp1qpp1p/8/1BpP2p1/8/2N2N2/PPPP1PPP/R1BQK2R b KQkq - 0 5"))
    }

    #[test]
    fn test_fen_2() {
        let mut test_board = Board::starting_pos();
        test_board.read_fen("8/3k4/1b1p4/1P3Rp1/6P1/4p2r/P3p2P/3K4 w - - 0 37".to_string());
        assert_eq!(test_board.fen.unwrap(), String::from("8/3k4/1b1p4/1P3Rp1/6P1/4p2r/P3p2P/3K4 w - - 0 37"))
    }
    #[test]
    fn test_fen_3() {
        let mut test_board = Board::starting_pos();
        test_board.read_fen("1r4r1/3Q2k1/8/3Np1bB/1p2P1Pp/P4P1p/2P1n2P/1R5K b - - 4 36".to_string());
        assert_eq!(test_board.fen.unwrap(), String::from("1r4r1/3Q2k1/8/3Np1bB/1p2P1Pp/P4P1p/2P1n2P/1R5K b - - 4 36"))
    }
}
