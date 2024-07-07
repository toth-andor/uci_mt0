use std::{
    io::{self, BufRead},
    thread,
};

use mateinzero as mi0;

fn sigmoid(secs: f32) -> f32 {
    (1.0 / (1.0 + std::f32::consts::E.powf(-(1.0 / 150.0) * (secs - 300.0)))) * 25.0 - 2.98007305055
}


fn main() {
    let mut eng = mi0::Game::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");

    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        let command;
        if let Ok(ltmp) = line {
            command = ltmp;
        } else {
            break;
        }

        if command == "move overhead" {
            println!("{}", 200);
        }

        let command_vec = command.split(" ").collect::<Vec<&str>>();

        if command_vec[0] == "uci" {
            println!("id name mi0");
            println!("id author to_dor");
            // println!("option name Move Overhead type string");
            // println!("option name Threads type spin");
            println!("uciok");
        }

        if command_vec[0] == "isready" {
            println!("readyok");
        }

        if command_vec[0] == "ucinewgame" {
            eng = mi0::Game::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        }

        if command_vec[0] == "position" {
            if command_vec.contains(&"startpos") {
                eng = mi0::Game::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
            }
            if command_vec.contains(&"fen") {
                let idx = command_vec.iter().position(|&f| f == "fen").unwrap();
                eng = mi0::Game::new(command_vec[idx + 1]);
            }
            if command_vec.contains(&"moves") {
                eng = mi0::Game::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
                let idx = command_vec.iter().position(|&f| f == "moves").unwrap();
                for i in idx + 1..command_vec.len() {
                    if let Ok(_) = &eng.do_move(command_vec[i].to_string()) {
                    } else {
                        break;
                    }
                }
            }
        }
        if command_vec.contains(&"go") {
            let mut thinktime = 6.5;

            if eng.is_white_turn() {
                if let Some(t) = command_vec.iter().position(|&t| t == "wtime") {
                    let av_time = command_vec[t + 1].parse::<f32>().unwrap() / 1000.0;
                    thinktime = sigmoid(av_time);
                }
            } else {
                if let Some(t) = command_vec.iter().position(|&t| t == "btime") {
                    let av_time = command_vec[t + 1].parse::<f32>().unwrap() / 1000.0;
                    thinktime = sigmoid(av_time);
                }
            }

            let mut e = eng.clone();
            thread::spawn(move || {
                println!("bestmove {}", e.best_move_secs(thinktime));
            });
        }
        if command_vec.contains(&"quit") {
            break;
        }
    }

    println!("Hello, world!");
}
