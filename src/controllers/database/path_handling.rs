use dirs_next::data_local_dir;
use std::path::PathBuf;

const LOCAL_DATA_DIR: &str = "unnamed_rss_manager";
const DB_FILE_NAME: &str = "unnamed_rss_manager.db";

pub fn get_local_data_directory() -> PathBuf {
    let path = data_local_dir().expect("Could not determine local data directory");
    return path;
}

pub fn initiate_db() {
    let mut path = get_local_data_directory();
    path.push(LOCAL_DATA_DIR);
    std::fs::create_dir_all(&path).expect("Could not create data directory");
    path.push(DB_FILE_NAME);
    std::fs::File::create(&path).expect("Could not create database file");
}

pub fn check_if_db_path_exists() {
    let exists = get_local_data_directory()
        .join(LOCAL_DATA_DIR)
        .join(DB_FILE_NAME)
        .exists();
    if !exists {
        initiate_db();
    }
}
