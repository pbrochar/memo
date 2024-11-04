use crate::memo::Memo;
use arboard::Clipboard;
use std::{error::Error, i64};
extern crate prettytable;
use prettytable::{format, row, Table};

pub struct MemoCommand;

impl MemoCommand {
    pub const ADD: &'static str = "add";
    pub const GET: &'static str = "get";
    pub const RM: &'static str = "rm";
    pub const LIST: &'static str = "ls";
    pub const SET: &'static str = "set";
    pub const COPY: &'static str = "cp";
}
pub struct MemoCommandHandler<'a> {
    pub memo: &'a mut Memo,
}

impl MemoCommandHandler<'_> {
    pub fn add(&mut self, key: &str, value: &str, ttl: Option<i64>) {
        match self.memo.get(key) {
            Some(_) => {
                println!("Key already exists: {}", key);
            }
            None => match self.memo.add(key, value, ttl) {
                Ok(_) => {
                    println!("Added key: {}", key);
                }
                Err(e) => {
                    eprintln!("Error adding key: {}", e);
                }
            },
        }
    }

    pub fn set(&mut self, key: &str, value: Option<&str>, ttl: Option<i64>) {
        let search_key = self.get_search_key(key);
        let key = search_key.as_str();

        match self.memo.get(key) {
            Some(_) => match self.memo.set(key, value, ttl) {
                Ok(_) => {
                    if value.is_some() {
                        println!("Setting key: {}", key);
                    }
                    if ttl.is_some() {
                        println!("Setting ttl for key: {}", key);
                    }
                }
                Err(e) => {
                    eprintln!("Error setting key: {}", e);
                }
            },
            None => {
                eprintln!("No value found for key: {}", key);
            }
        }
    }

    pub fn copy(&mut self, key: &str) -> Result<(), Box<dyn Error>> {
        let search_key = self.get_search_key(key);
        let key = search_key.as_str();

        match self.memo.get(key) {
            Some(v) => {
                let value = &v.value;
                let mut clipboard = Clipboard::new()?;
                clipboard.set_text(value)?;
            }
            None => {
                eprintln!("No value found for key: {}", key);
            }
        }
        Ok(())
    }

    pub fn get(&mut self, key: &str, to_clipboard: bool) -> Result<(), Box<dyn Error>> {
        let search_key = self.get_search_key(key);

        let key = search_key.as_str();

        match self.memo.get(key) {
            Some(v) => {
                let value = &v.value;
                if to_clipboard {
                    let mut clipboard = Clipboard::new()?;
                    clipboard.set_text(value)?;
                }
                println!("{}", value);
            }
            None => {
                eprintln!("No value found for key: {}", key);
            }
        }
        Ok(())
    }
    pub fn rm(&mut self, key: &str) -> Result<(), Box<dyn Error>> {
        let search_key = self.get_search_key(key);
        let key = search_key.as_str();

        match self.memo.get(key) {
            Some(_) => {
                self.memo.rm(key)?;
                println!("Removing key: {}", key);
            }
            None => {
                eprintln!("No value found for key: {}", key);
            }
        }
        Ok(())
    }

    pub fn list(&self, pretty: bool) {
        if pretty {
            let mut table = Table::new();
            let mut readable_ttl: String;

            table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
            table.set_titles(row!["Key", "Value", "TTL"]);

            for (key, value) in &self.memo.store {
                match &value.ttl {
                    Some(ttl) => {
                        let lifetime = ttl.parse::<i64>().unwrap() - chrono::Utc::now().timestamp();
                        readable_ttl = lifetime.to_string() + "s";
                    }
                    None => {
                        readable_ttl = "".to_string();
                    }
                }

                table.add_row(row![key, value.value, readable_ttl]);
            }

            table.printstd();
        } else {
            for (key, value) in &self.memo.store {
                println!("{} : {}", key, value.value);
            }
        }
    }

    fn get_search_key(&self, key: &str) -> String {
        if key == "-" && self.memo.meta.last_key_used.is_some() {
            if let Some(last_key) = &self.memo.meta.last_key_used {
                return last_key.to_string();
            }
        }
        key.to_string()
    }
}
