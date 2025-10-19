use dirs_next::data_local_dir;
use std::path::PathBuf;

pub const LOCAL_DATA_DIR: &str = "reelay";
pub const DB_FILE_NAME: &str = "reelay.db";

pub fn get_local_app_data_directory() -> PathBuf {
    let path = data_local_dir().expect("Could not determine local data directory");
    return path;
}

pub fn get_complete_db_path() -> PathBuf {
    let mut path = get_local_app_data_directory();
    path.push(LOCAL_DATA_DIR);
    path.push(DB_FILE_NAME);
    return path;
}

pub fn create_db_file_structure() {
    let mut path = super::path_handling::get_local_app_data_directory();
    path.push(LOCAL_DATA_DIR);
    std::fs::create_dir_all(&path).expect("Could not create data directory");
    path.push(DB_FILE_NAME);
    std::fs::File::create(&path).expect("Could not create database file");
}

pub fn check_if_db_exists_or_initiate() {
    let exists = super::path_handling::get_complete_db_path().exists();
    if !exists {
        super::path_handling::create_db_file_structure();
        // TODO: t0wt13m5 - handle error properly
        // TODO: t0wt13m5 - revisit this logic
        let _ = super::operations::initiate_db(
            &super::connection::create_db_connection().expect("Failed to create DB connection"),
        );
    }
}
