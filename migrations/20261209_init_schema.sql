CREATE TABLE IF NOT EXISTS users (
    user_id INTEGER PRIMARY KEY NOT NULL,
    username TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    file_quota_bytes INTEGER NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS files (
    file_id INTEGER PRIMARY KEY NOT NULL,
    hash_filename TEXT UNIQUE NOT NULL,
    mimetype TEXT NOT NULL,
    file_size_bytes INTEGER NOT NULL,
    uploaded_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS user_files (
    uploaded_by INTEGER NOT NULL,
    file_id INTEGER NOT NULL,
    original_filename TEXT NOT NULL,
    uploaded_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (uploaded_by) REFERENCES users(user_id),
    FOREIGN KEY (file_id) REFERENCES files(file_id),
    PRIMARY KEY (uploaded_by, file_id)
);
