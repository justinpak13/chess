use std::ops::Index;

fn main() {
    let mut board = Board::starting_pos();
    
    //let test_fen = String::from("8/8/8/8/8/8/8/Q7 w - - 0 20");
    let test_fen = String::from("8/8/8/8/3Q4/8/8/8 w - - 0 20");

    board.read_fen(test_fen);
    board.print_board();

    println!("{:?}",board.get_moves());

    //test
    //

    
}


#[derive(Clone,Debug,PartialEq)]
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

#[derive(Debug,Clone)]
struct Square {
    piece: Piece,
    x_coord: char, 
    y_coord: u8,
    color: char
}

#[derive(Debug)]
struct Board {
    squares: Vec<Vec<Square>>,
    color_move: char,
    caslting_rights: String,
    half_moves: i8,
    full_moves: i8,
    fen: Option<String>

}

impl Board {
    fn _new() -> Board  {
    // need to generate square information then put into the squares trait

    let mut vector: Vec<Vec<Square>> = vec![];
    for i in (1..9).rev() {
        let mut holder: Vec<Square> = vec![];
        for (index, j) in  ('A'..'I').enumerate(){
            holder.push(Square {piece: Piece::Blank, x_coord: j, y_coord: i, color: {if i % 2 == (index % 2) as u8 {'w'} else {'b'}}});
        }
        vector.push(holder);
    }

        let result:Board = Board { squares: vector, color_move: 'w', caslting_rights: String::from("KQkq"), half_moves: 0, full_moves: 0, fen: None } ;
        result
    }

    fn read_fen(&mut self, fen: String){

        let fen_split: Vec<&str> = fen.split(" ").collect();
        self.color_move = fen_split[1].parse().unwrap();
        self.caslting_rights = String::from(fen_split[2]);
        self.half_moves = fen_split[4].parse().unwrap();
        self.full_moves = fen_split[5].parse().unwrap();

        let coordinates: Vec<&str> = fen_split[0].split("/").collect();

        let mut vector: Vec<Vec<Square>> = vec![];
        for i in (1..9).rev() {
            let mut holder: Vec<Square> = vec![];
            for (index, j) in  ('A'..'I').enumerate(){
                holder.push(Square {piece: Piece::Blank, x_coord: j, y_coord: i, color: {if i % 2 == (index % 2) as u8 {'w'} else {'b'}}});
            }
            vector.push(holder);
        }
        self.squares = vector; 

        for i in 0..8{
            let row = i;
            let mut counter: u32 = 0;

            for j in coordinates[row].chars().into_iter(){
                if counter > 8 {
                    break
                }

                match j {
                    'r' => self.squares[i][counter as usize].piece = Piece::RookBlack,
                    'n' => self.squares[i][counter as usize].piece = Piece::KnightBlack,
                    'b' => self.squares[i][counter as usize].piece = Piece::BishopBlack,
                    'q' => self.squares[i][counter as usize].piece = Piece::QueenBlack,
                    'k' => self.squares[i][counter as usize].piece = Piece::KingBlack,
                    'p' => self.squares[i][counter as usize].piece = Piece::PawnBlack,
                    'R' => self.squares[i][counter as usize].piece = Piece::RookWhite,
                    'N' => self.squares[i][counter as usize].piece = Piece::KnightWhite,
                    'B' => self.squares[i][counter as usize].piece = Piece::BishopWhite,
                    'Q' => self.squares[i][counter as usize].piece = Piece::QueenWhite,
                    'K' => self.squares[i][counter as usize].piece = Piece::KingWhite,
                    'P' => self.squares[i][counter as usize].piece = Piece::PawnWhite,
                    _ => {
                        for _ in 0..j.to_digit(10).unwrap() - 1{
                            self.squares[i][counter as usize].piece = Piece::Blank;
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
        let mut numbers = (1..9).rev().into_iter();


        for rows in self.squares.clone().into_iter(){
            print!("{:?}",numbers.next().unwrap());
            for columns in rows {
                print!("|");
                match columns.piece {
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
        print!(" ");
        for letter in 'A'..'I' {
            print!(" {}", letter);
        }
        println!("");
    } 
    
    fn get_moves(&self)  -> Vec<String>{
        let valid_pieces = match self.color_move {
            'w' => vec![Piece::RookWhite, Piece::KingWhite, Piece::KnightWhite, Piece::QueenWhite, Piece::BishopWhite, Piece::PawnWhite],
            'b' => vec![Piece::RookBlack, Piece::KingBlack, Piece::KnightBlack, Piece::QueenBlack, Piece::BishopBlack, Piece::PawnBlack],
            _ => vec![Piece::Blank]
        };

        let mut result: Vec<String> = vec![];

        for row in &self.squares {
            for column in row {
                if valid_pieces.contains(&column.piece) {

                    let mut vector= ('A'..'I').into_iter();

                    match column.piece {
                        Piece::QueenBlack | Piece::QueenWhite => {
                            let x_point = vector.position(|x| x == column.x_coord).unwrap();

                            // up and down 
                            let up_iter = (0..column.y_coord).rev();
                            
                            for i in up_iter {
                                let mut result_string = String::new();
                                if self.squares[i as usize][x_point].piece != Piece::Blank {
                                    // need to add functionality for opponent and player pieces 
                                };
                                result_string.push(column.x_coord);
                                result_string.push_str(&self.squares[i as usize][x_point].y_coord.to_string());

                                result.push(result_string);
                            }

                            let down_iter = (column.y_coord + 1)..8;

                            for i in down_iter {
                                let mut result_string = String::new();
                                if self.squares[i as usize][x_point].piece != Piece::Blank {
                                    // need to add functionality for opponent and player pieces 
                                };
                                result_string.push(column.x_coord);
                                result_string.push_str(&self.squares[i as usize][x_point].y_coord.to_string());

                                result.push(result_string);
                            }
                            
                            // left and right 
                            let left_iter = (0..x_point).rev();


                            let right_iter = (x_point + 1)..8;



                            
                            // diagonal
                        },
                        _ => {}

                    }
                    
                }
            }
        };

        //println!("{}",result.len());
        result

    }

}


fn get_fen(board: &Board) -> String {
    let mut result = String::new();
    
    let mut counter = 0;
    for row in &board.squares {
        for column in row {
            match column.piece {
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
