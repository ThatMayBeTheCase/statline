mod database;
mod game;
mod utils;

use game::Game;
use utils::format_duration;

use std::thread;
use std::time::Duration;
use sysinfo::{ProcessesToUpdate, System};

fn main() {
    // Spel som ska kollas
    let mut games = vec![
        Game::new("LocalSend".to_string(), "localsend_app.exe".to_string()),
        Game::new("Calculator".to_string(), "CalculatorApp.exe".to_string()),
    ];

    //Databas
    let conn = database::init_database().expect("Failed to initialize database");

    database::register_games(&conn, &games).expect("Failed to register games");

    for game in games.iter_mut() {
        let game_id =
            database::get_game_id(&conn, &game.executable).expect("Failed to get game ID");

        game.total_playtime =
            database::get_total_playtime(&conn, game_id).expect("Failed to load total playtime");

        println!(
            "{} total playtime: {}",
            game.name,
            format_duration(game.total_playtime)
        );
    }

    let mut sys = System::new();

    println!("STATLINE - Process Monitor");
    println!("--------------------------");

    loop {
        sys.refresh_processes(ProcessesToUpdate::All, true);

        for game in games.iter_mut() {
            let found = is_process_running(&sys, game.executable.as_str());

            if found && !game.was_running {
                game.start_session();
            }

            if !found && game.was_running {
                if let Some(session) = game.stop_session() {
                    let game_id = database::get_game_id(&conn, &game.executable)
                        .expect("Failed to get game ID");

                    database::save_session(
                        &conn,
                        game_id,
                        session.started_at,
                        session.ended_at,
                        session.duration_seconds,
                    )
                    .expect("Failed to save session");

                    println!("{} Stopped!", game.name);
                    println!(
                        "Playtime: {}",
                        format_duration(session.duration_seconds as u64)
                    );
                    println!("Total Playtime: {}", format_duration(game.total_playtime));
                }
            }
        }

        thread::sleep(Duration::from_secs(5));
    }
}

// Funktion, kollar om processen är igång
fn is_process_running(sys: &System, process_name: &str) -> bool {
    for process in sys.processes().values() {
        if process
            .name()
            .to_string_lossy()
            .eq_ignore_ascii_case(process_name)
        {
            return true;
        }
    }
    false
}
