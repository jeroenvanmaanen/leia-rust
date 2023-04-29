use std::ffi::OsStr;
use std::fs::{OpenOptions,create_dir_all};
use std::io::{Seek, SeekFrom};
use std::path::PathBuf;
use anyhow::{Result,anyhow};

pub fn compute_oracle(mut oracle_path: PathBuf, file: &str, limit: &Option<u64>) -> Result<()> {
    let file_path = PathBuf::from(file);
    if file_path.is_absolute() {
        oracle_path = file_path;
    } else {
        oracle_path.push(file_path);
    }
    println!("Oracle file: {:?} (limit: {:?})", oracle_path, limit);
    let path_clone = oracle_path.clone();
    let dir_path = path_clone.parent().ok_or(anyhow!("Which dir?"))?;
    println!("Temp dir: {:?}", dir_path);
    create_dir_all(dir_path)?;

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(oracle_path)?;
    file.seek(SeekFrom::Start(0))?;
    Ok(())
}

pub fn get_project_dir() -> Result<PathBuf> {
    let mut oracle_path = std::env::current_exe()?;
    let mut file_name = oracle_path.file_name().and_then(OsStr::to_str).unwrap_or("").to_string();
    while oracle_path.pop() {
        if file_name == "target" {
            break;
        }
        file_name = oracle_path.file_name().and_then(OsStr::to_str).unwrap_or("").to_string();
    }
    println!("Project dir: {:?}", oracle_path);
    Ok(oracle_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use lazy_static::lazy_static;
    use uuid::Uuid;

    lazy_static! {
        static ref TEST_TMP: String = Uuid::new_v4().simple().encode_lower(&mut Uuid::encode_buffer()).to_string();
    }

    #[test]
    fn test() {
        let project_dir = get_project_dir().unwrap();
        let file = format!("compute-oracle/data/tmp/{}/empty.yaml", *TEST_TMP);
        compute_oracle(project_dir, &file, &Some(0)).unwrap();
    }
}