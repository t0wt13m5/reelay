use super::path_handling::{get_local_data_directory, DB_FILE_NAME, LOCAL_DATA_DIR};

pub fn initiate_db() {
    let mut path = get_local_data_directory();
    path.push(LOCAL_DATA_DIR);
    std::fs::create_dir_all(&path).expect("Could not create data directory");
    path.push(DB_FILE_NAME);
    std::fs::File::create(&path).expect("Could not create database file");
}
