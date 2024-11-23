pub mod cli;
pub mod commands;
pub mod memo;

use cli::{cli, MemoArg};
use commands::{MemoCommand, MemoCommandHandler};
use memo::Memo;
use std::error::Error;

fn key_is_valid(key: &str) -> bool {
    if key.is_empty() {
        eprintln!("Key cannot be empty");
        return false;
    }
    if key.contains(' ') {
        eprintln!("Key cannot contain spaces");
        return false;
    }
    if !key.starts_with(|c: char| c.is_ascii_alphabetic()) {
        eprintln!("Key must start with an alphabetic character");
        return false;
    }
    true
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut memo = Memo::get_default().expect("Could not load memo file");

    memo.flush_ttl_values()?;

    let mut command = MemoCommandHandler { memo: &mut memo };
    match cli().get_matches().subcommand() {
        Some((MemoCommand::ADD, sub_matches)) => {
            let key: &String = sub_matches.get_one(MemoArg::KEY).unwrap();
            let value: &String = sub_matches.get_one(MemoArg::VALUE).unwrap();
            let ttl: Option<i64> = sub_matches
                .get_one(MemoArg::TTL)
                .map(|t| chrono::Utc::now().timestamp() + t);

            if !key_is_valid(key) {
                return Ok(());
            }
            command.add(key, value, ttl);
        }

        Some((MemoCommand::GET, sub_matches)) => {
            let key: &String = sub_matches.get_one(MemoArg::KEY).expect("KEY is required");
            let to_clipboard = sub_matches.get_flag("clipboard");
            command.get(key, to_clipboard)?;
        }

        Some((MemoCommand::RM, sub_matches)) => {
            let key: &String = sub_matches.get_one(MemoArg::KEY).expect("KEY is required");
            command.rm(key)?;
        }
        Some((MemoCommand::LIST, sub_matches)) => {
            let pretty = sub_matches.get_flag("pretty");
            let ttl = sub_matches.get_flag("ttl");
            let created = sub_matches.get_flag("created");
            command.list(pretty, ttl, created);
        }
        Some((MemoCommand::SET, sub_matches)) => {
            let key: &String = sub_matches.get_one(MemoArg::KEY).unwrap();

            let ttl: Option<i64> = sub_matches
                .get_one(MemoArg::TTL)
                .map(|t| chrono::Utc::now().timestamp() + t);
            let value: Option<&String> = sub_matches.get_one(MemoArg::VALUE);
            command.set(key, value.map(|v| v.as_str()), ttl);
        }
        Some((MemoCommand::COPY, sub_matches)) => {
            let key: &String = sub_matches.get_one(MemoArg::KEY).expect("KEY is required");
            command.copy(key)?;
        }
        Some(("_complete", sub_matches)) => {
            let default = "".to_string();
            let word: &String = sub_matches.get_one(MemoArg::KEY).unwrap_or(&default);
            let mut suggestions = Vec::new();

            for key in command.memo.store.keys() {
                if key.starts_with(word) {
                    suggestions.push(key.to_string());
                }
            }
            for suggestion in suggestions {
                println!("{}", suggestion);
            }
        }

        _ => {
            println!("No subcommand found");
        }
    }
    Ok(())
}
