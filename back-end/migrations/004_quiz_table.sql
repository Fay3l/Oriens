CREATE TABLE IF NOT EXISTS quiz (
    id VARCHAR(30) NOT NULL,
    adjectif VARCHAR(255) NOT NULL,
    description TEXT,
    formations JSONB DEFAULT '[]' NOT NULL,
    metiers JSONB DEFAULT '[]' NOT NULL,
    softskills JSONB DEFAULT '[]' NOT NULL,
    completed_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);