use crate::memo::{Memo, MemoVariable};
use arboard::Clipboard;
use chrono::DateTime;
use std::{error::Error, i64};
extern crate prettytable;
use prettytable::{format, Row, Table};

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

    fn format_ttl(ttl_value: &Option<String>) -> String {
        match ttl_value {
            Some(ttl) => {
                let lifetime = ttl.parse::<i64>().unwrap_or(0) - chrono::Utc::now().timestamp();
                format!("{}s", lifetime)
            }
            None => "null".to_string(),
        }
    }

    fn get_row(&self, key: &str, value: &MemoVariable, ttl: bool, created: bool) -> Vec<String> {
        let mut row = vec![key.to_string(), value.value.clone()];

        if created {
            let created_at = DateTime::from_timestamp(value.created_at, 0)
                .map(|dt| dt.to_string())
                .unwrap_or_else(|| "Invalid date".to_string());
            row.push(created_at);
        }

        if ttl {
            row.push(Self::format_ttl(&value.ttl));
        }

        row
    }

    fn get_title(ttl: bool, created: bool) -> Vec<String> {
        let mut title = vec!["Key".to_string(), "Value".to_string()];

        if created {
            title.push("Created".to_string());
        }
        if ttl {
            title.push("TTL".to_string());
        }

        title
    }

    pub fn list(&self, pretty: bool, ttl: bool, created: bool) {
        let title = Self::get_title(ttl, created);

        if pretty {
            let mut table = Table::new();
            table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
            let title_refs: Vec<&str> = title.iter().map(|s| s.as_str()).collect();
            table.set_titles(Row::from(title_refs));

            for (key, value) in &self.memo.store {
                let row = self.get_row(key, value, ttl, created);
                let row_refs: Vec<&str> = row.iter().map(|s| s.as_str()).collect();
                table.add_row(Row::from(row_refs));
            }
            table.printstd();
        } else {
            for (key, value) in &self.memo.store {
                let row = self.get_row(key, value, ttl, created);
                println!("{}", row.join(" - "));
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
