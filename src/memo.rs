use home::home_dir;
use serde::{Deserialize, Serialize};
use serde_json::{json, to_writer_pretty};
use std::collections::HashMap;
use std::fs::{self, File};
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
pub struct MemoVariable {
    pub value: String,
    pub ttl: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MemoMeta {
    pub last_key_used: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Memo {
    pub store: HashMap<String, MemoVariable>,
    pub meta: MemoMeta,
    #[serde(skip)]
    file_path: PathBuf,
}

impl Memo {
    pub fn get(&mut self, key: &str) -> Option<&MemoVariable> {
        match self.store.get(key) {
            Some(v) => {
                self.meta.last_key_used = Some(key.to_string());
                self.write_to_file().expect("Could not write to file");
                Some(v)
            }
            None => None,
        }
    }

    pub fn add(
        &mut self,
        key: &str,
        value: &str,
        ttl: Option<i64>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let ttl = ttl.map(|t| t.to_string());
        self.store.insert(
            key.to_string(),
            MemoVariable {
                value: value.to_string(),
                ttl,
            },
        );
        self.meta.last_key_used = Some(key.to_string());
        self.write_to_file()?;
        Ok(())
    }

    pub fn set(
        &mut self,
        key: &str,
        value: Option<&str>,
        ttl: Option<i64>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let ttl = ttl.map(|t| t.to_string());
        if let Some(v) = self.store.get_mut(key) {
            if let Some(value) = value {
                v.value = value.to_string();
            }
            if let Some(ttl) = ttl {
                v.ttl = Some(ttl);
            }
        }
        self.meta.last_key_used = Some(key.to_string());
        self.write_to_file()?;
        Ok(())
    }
    pub fn rm(&mut self, key: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.store.remove(key);
        self.write_to_file()?;
        Ok(())
    }

    fn from_file_path(file_path: &PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let file = File::open(file_path)?;
        Ok(serde_json::from_reader(file)?)
    }

    pub fn get_default() -> Result<Self, Box<dyn std::error::Error>> {
        let file_path = Self::ensure_directory_and_file(Self::get_memo_dir()?, "default.json")?;
        let memo = Self::from_file_path(&file_path)?;

        Ok(Self {
            store: memo.store,
            meta: memo.meta,
            file_path,
        })
    }

    fn write_to_file(&self) -> Result<(), Box<dyn std::error::Error>> {
        serde_json::to_writer_pretty(&File::create(&self.file_path)?, &self)?;
        Ok(())
    }
    fn ensure_directory_and_file(
        directory_path: PathBuf,
        filename: &str,
    ) -> Result<PathBuf, Box<dyn std::error::Error>> {
        if !directory_path.exists() {
            fs::create_dir_all(&directory_path)?;
        }

        let file_path = directory_path.join(filename);

        if !file_path.exists() {
            let default = json!({
                "store": {},
                "meta": {
                    "last_key_used": ""
                }
            });
            let file = File::create(&file_path)?;

            to_writer_pretty(file, &default)?;
        }

        Ok(file_path)
    }

    fn get_memo_dir() -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
        let home_dir = home_dir()
            .ok_or("Could not find home directory")?
            .join(".memo");
        Ok(home_dir)
    }

    pub fn flush_ttl_values(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let now = chrono::Utc::now().timestamp();
        self.store.retain(|_, v| {
            if let Some(ttl) = v.ttl.as_ref() {
                if let Ok(ttl) = ttl.parse::<i64>() {
                    return ttl >= now;
                }
            }
            true
        });
        self.write_to_file()?;
        Ok(())
    }
}
