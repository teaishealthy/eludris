CREATE TABLE IF NOT EXISTS files (
  id BIGINT PRIMARY KEY,
  file_id BIGINT NOT NULL,
  name VARCHAR(256) NOT NULL,
  content_type VARCHAR(32) NOT NULL,
  hash VARCHAR(64) NOT NULL,
  bucket VARCHAR(32) NOT NULL,
  spoiler BOOLEAN NOT NULL DEFAULT FALSE,
  width INTEGER,
  height INTEGER
);
