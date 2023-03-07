use crate::color::Color;
use crate::game::Game;
use crate::moves::StraightStrike;
use crate::piece::Piece;
include!("lib.rs");

#[derive(Debug)]
struct MoveAsStrike {
    from: usize,
    to: usize,
    take: usize,
}
#[derive(Debug)]
struct MoveAsQuite {
    from: usize,
    to: usize,
}

pub fn best_move_triangle() {
    let mut game = Game::new(8);
    game.current_position.next_move = Some(Color::Black);
    vec![29].iter()
        .for_each(|pos|
            game.insert_piece(Piece::new(game.to_pack(*pos), Color::White, true)));
    vec![0, 18, 9].iter()
        .for_each(|pos|
            game.insert_piece(Piece::new(game.to_pack(*pos), Color::Black, true)));

    use crate::moves::PieceMove;
    while game.finish_check().is_none() {
        print!("state {}\n", game.state_());
        print!("history {:?}\n", game.position_history.len());
        let best = game.get_best_move_rust();
        print!("{}", {
            if best.get_move_item().strike.is_some() {
                format!("move: {:?}\n", best.get_move_item().strike.unwrap().vec.iter().map(
                    |x|
                        MoveAsStrike {
                            from: game.to_board(x.from()),
                            to: game.to_board(x.to()),
                            take: game.to_board(x.take().unwrap()),
                        }).collect::<Vec<_>>())
            } else {
                format!("move: {:?}\n", [best.get_move_item().mov.unwrap()].iter().map(|x|
                MoveAsQuite{
                    from: game.to_board(x.from()),
                    to: game.to_board(x.to)
                }).collect::<Vec<_>>())
            }
        });
        game.make_move_by_pos_item(&best);
    }
}

pub fn main() {
    best_move_triangle();
    let mut game = Game::new(8);
    game.insert_piece(Piece::new(22, Color::White, false));
    game.insert_piece(Piece::new(4, Color::Black, true));
    game.insert_piece(Piece::new(21, Color::Black, true));
    game.insert_piece(Piece::new(20, Color::Black, true));
    game.insert_piece(Piece::new(12, Color::Black, true));
    game.insert_piece(Piece::new(13, Color::Black, true));
    game.insert_piece(Piece::new(26, Color::Black, true));
    game.current_position.next_move = Some(Color::White);
    let now = Instant::now();
    for _i in 0..1000000 {
        let mut list = game.current_position.get_move_list(false);
        let mut pos_list: Vec<_> = {
            list.list.iter_mut().map(|x| {
                let mut pos = game.current_position.make_move_and_get_position(x);
                game.current_position.unmake_move(x);
                pos.position.evaluate();
                pos
            }).collect()
        };
        pos_list.sort_by_key(|x|
            x.position.eval.unwrap() * if x.position.next_move.unwrap() == Color::White { -1 } else { 1 });
        let po = game.current_position.make_move_and_get_position(&mut list.list[0]);
        game.finish_check();
        if po != po { break; }
        game.current_position.unmake_move(&mut list.list[0]);
    }
    print!("strike:  {:.2?}\n", now.elapsed());

    let mut game = Game::new(8);
    game.insert_piece(Piece::new(game.to_pack(47), Color::White, false));
    game.insert_piece(Piece::new(game.to_pack(63), Color::White, false));
    game.insert_piece(Piece::new(game.to_pack(15), Color::White, true));
    vec![54, 43, 20].iter()
        .for_each(|pos|
            game.insert_piece(Piece::new(game.to_pack(*pos), Color::Black, false)));
    game.current_position.next_move = Some(Color::White);
    use std::time::Instant;
    let now = Instant::now();
    for _i in 0..1000000 {
        let mut list = game.current_position.get_move_list(false);
        let po = game.current_position.make_move_and_get_position(&mut list.list[0]);
        if po != po { break; }
        game.current_position.unmake_move(&mut list.list[0]);
    }
    print!("strike 2:  {:.2?}\n", now.elapsed());


    let mut game = Game::new(8);
    game.insert_piece(Piece::new(game.to_pack(16), Color::White, false));
    game.insert_piece(Piece::new(game.to_pack(18), Color::White, false));
    game.insert_piece(Piece::new(game.to_pack(20), Color::White, false));
    game.insert_piece(Piece::new(game.to_pack(22), Color::White, false));
    game.current_position.next_move = Some(Color::White);

    let now = Instant::now();
    for _i in 0..1000000 {
        let mut list = game.current_position.get_move_list(false);
        let po = game.current_position.make_move_and_get_position(&mut list.list[0]);
        if po != po { break; }
        game.current_position.unmake_move(&mut list.list[0]);
    }
    print!("simple: {:.2?}\n", now.elapsed());
}