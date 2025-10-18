use super::operations::initiate_db;
use dirs_next::data_local_dir;
use std::path::PathBuf;

pub const LOCAL_DATA_DIR: &str = "unnamed_rss_manager";
pub const DB_FILE_NAME: &str = "unnamed_rss_manager.db";

pub fn get_local_data_directory() -> PathBuf {
    let path = data_local_dir().expect("Could not determine local data directory");
    return path;
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
