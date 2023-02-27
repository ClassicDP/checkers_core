use crate::color::Color;
use crate::game::Game;
use crate::piece::Piece;
include!("lib.rs");


pub fn main() {
    let mut game = Game::new(8);
    let ref mut pos = game.current_position;
    pos.inset_piece(Piece::new(22, Color::White, false));
    pos.inset_piece(Piece::new(4, Color::Black, true));
    pos.inset_piece(Piece::new(21, Color::Black, true));
    pos.inset_piece(Piece::new(20, Color::Black, true));
    pos.inset_piece(Piece::new(12, Color::Black, true));
    pos.inset_piece(Piece::new(13, Color::Black, true));
    pos.inset_piece(Piece::new(26, Color::Black, true));

    let now = Instant::now();
    for _i in 0..1000000 {
        let mut list = pos.get_move_list(Color::Black, false);
        let po = pos.make_move_and_get_position(&mut list.list[0]);
        if po != po { break; }
        pos.unmake_move(&mut list.list[0]);
    }
    print!("strike: ход  {:.2?}\n", now.elapsed());

    let mut game = Game::new(8);
    game.insert_piece(Piece::new(game.to_pack(47), Color::White, false));
    game.insert_piece(Piece::new(game.to_pack(63), Color::White, false));
    game.insert_piece(Piece::new(game.to_pack(15), Color::White, true));
    vec![54, 43, 20].iter()
        .for_each(|pos|
            game.insert_piece(Piece::new(game.to_pack(*pos), Color::Black, false)));

    use std::time::Instant;
    let now = Instant::now();
    for _i in 0..1000000 {
        let mut list = pos.get_move_list(Color::White, false);
        let po = pos.make_move_and_get_position(&mut list.list[0]);
        if po != po { break; }
        pos.unmake_move(&mut list.list[0]);
    }
    print!("{:.2?}", now.elapsed());
}