extern crate tensorflow;
mod game;
mod piece;
mod king;
mod queen;
mod rook;
mod bishop;
mod knight;
mod pawn;
mod player;
mod model;
mod args;


use std::{collections::HashMap, sync::{Mutex, Arc}};

// use std::process::Command;
use args::ChessArgs;
use clap::Parser;
use game::Game;
use model::Model;
use player::Player;
use game::DEFAULT_EPSILON_DECAY;

fn main() {
    let args = ChessArgs::parse();
    
    match args.game_type {
        args::GameType::TwoPlayer(args) => {
            two_player_game(args.allow_hints);
        },
        args::GameType::SinglePlayer(args) => {
            single_player_game(args.black, args.heuristic, args.search_depth, args.model_dir);
        },
        args::GameType::SelfPlay(args) => {
            self_play_games(args.heuristic, args.search_depth, args.num_games, args.model_dir, args.epsilon_greedy, args.epsilon_decay);
        }
    }
}

fn two_player_game(allow_hints: bool) {
    let mut game = Game::two_player_game(allow_hints);
    let mut play_again = true;
    while play_again {
        launch_game(&mut game);
        play_again = user_play_again();
    }
}

fn single_player_game(black: bool, heuristic: bool, search_depth: Option<u8>, model_dir: Option<String>) {
    let computer_player = if black {Some(Player::One)} else {Some(Player::Two)};
    let model = if heuristic { None } else { Some(Model::new(model_dir)) };
    let mut game = Game::single_player_game(computer_player, model.as_ref(), search_depth);
    games_loop(&mut game);
}

fn self_play_games(heuristic: bool, search_depth: Option<u8>, num_games: u16, model_dir: Option<String>, epsilon_greedy: bool, epsilon_decay: Option<f64>) {
    let model = if heuristic { None } else { Some(Model::new(model_dir)) };
    let mut white_wins = 0;
    let mut black_wins = 0;
    let mut draws = 0;
    let (mut epsilon, epsilon_decay) = if epsilon_greedy {
        (Some(1.0), Some(epsilon_decay.unwrap_or(DEFAULT_EPSILON_DECAY)))
    } else {
        (None, None)
    };
    let cache = Arc::new(Mutex::new(HashMap::new()));
    let start = std::time::Instant::now();
    for i in 1..num_games+1 {
        if num_games > 1 {
            println!("Playing game {}/{}", i, num_games);
        }
        let mut game = Game::self_play(model.as_ref(), search_depth, epsilon_greedy, epsilon, epsilon_decay, Some(cache.clone()));
        let now = std::time::Instant::now();
        launch_game(&mut game);
        let elapsed = now.elapsed();
        println!("Time to play game {}: {:?}", i, elapsed);
        if let Some(ep) = epsilon {
            epsilon = Some(ep*epsilon_decay.unwrap());
            println!("Epsilon: {}", ep);
        }
        match game.winner() {
            Some(Player::One) => white_wins += 1,
            Some(Player::Two) => black_wins += 1,
            None => draws += 1
        }
        if !heuristic {
            game.save_model();
        }
    }
    if num_games > 1 {
        let elapsed = start.elapsed();
        println!("Total time for {} games: {:?}", num_games, elapsed);
        println!("White wins: {}", white_wins);
        println!("Black wins: {}", black_wins);
        println!("Draws: {}", draws);
    }
}

fn games_loop(game: &mut Game) {
    let mut play_again = true;
    while play_again {
        launch_game(game);
        play_again = user_play_again();
    }
}

fn launch_game(game: &mut Game) {
    let mut game_over = false;
    while !game_over {
        // print!("\x1b[120S\x1b[1;1H");
        // print!("\x1B[2J\x1B[1;1H");
        // Command::new(if cfg!(target_os = "windows") {"cls"} else {"clear"}).status().unwrap();
        game_over = game.turn();
    }
}

fn user_play_again() -> bool {
    println!("Play again? (y/n)");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_ascii_lowercase() == "y"
}

// fn main() {
//     let args = ChessArgs::parse();

//     let computer_player = if args.two_player {None} else {if args.black {Some(Player::One)} else {Some(Player::Two)}};
//     if args.heuristic && args.self_play {
//         return heuristic_self_play(args.search_depth, args.num_games);
//     }
    
//     let save_dir = args.save_dir;
//     let model = if args.two_player || args.heuristic { None } else { Some(Model::new(model_dir.as_str())) };
//     if args.self_play {
//         reinforcement_learning(args.num_games, &model, args.search_depth, args.epsilon_greedy, args.epsilon_decay);
//     } else {
//         let mut play_again = true;
//         while play_again {
//             play_again = launch_game(args.two_player, computer_player, &model, args.search_depth, args.allow_hints);
//         }
//     }
// }

// fn launch_game(two_player: bool, computer_player: Option<Player>, model: &Option<Model>, search_depth: Option<u8>, allow_hints: bool) -> bool {
//     let mut game = if two_player {
//         Game::two_player_game(allow_hints)
//     } else {
//         Game::single_player_game(computer_player, model, search_depth)
//     };

//     let mut game_over = false;
//     while !game_over {
//         // print!("\x1b[120S\x1b[1;1H");
//         // print!("\x1B[2J\x1B[1;1H");
//         // Command::new(if cfg!(target_os = "windows") {"cls"} else {"clear"}).status().unwrap();
//         game_over = game.turn();
//     }    
//     play_again()
// }