pub fn temp_db_path() -> (tempfile::TempDir, String) {
    let db_folder = tempfile::tempdir().unwrap();
    let db_path = db_folder
        .path()
        .join("test.db")
        .to_string_lossy()
        .to_string();
    (db_folder, db_path)
}
