use dirs_next::data_local_dir;
// use std::fs;
// use std::path::Path;
use std::path::PathBuf;

const LOCAL_DATA_DIR: &str = "unnamed_rss_manager";
// const DB_FILE_NAME: &str = "unnamesd_rss_manager.db";

pub fn get_local_data_directory() -> PathBuf {
    let path = data_local_dir().expect("Could not determine local data directory");
    return path;
}

// pub fn initiate_db_directory() {
//     let mut path = get_local_data_directory();
//     path.push(LOCAL_DATA_DIR);
//     std::fs::create_dir_all((&path)).expect("Could not create data directory");
//     // TODO: t0wt13m5 potentially .push("feeds.db") and return full path
// }

pub fn check_if_db_path_exists() -> bool {
    // let mut path = get_local_data_directory();
    // path.push(LOCAL_DATA_DIR);
    // if path.exists() {
    //     return true;
    // } else {
    //     return false;
    // }
    let exists = get_local_data_directory()
        .join(LOCAL_DATA_DIR) // returns a new PathBuf
        .exists();
    return exists;
}
