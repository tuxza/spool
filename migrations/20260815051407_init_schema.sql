CREATE TABLE IF NOT EXISTS files (
    fid INTEGER PRIMARY KEY NOT NULL, -- file id ofc
    hash_filename TEXT UNIQUE NOT NULL,
    uploaded_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    reference_count INTEGER NOT NULL
);
