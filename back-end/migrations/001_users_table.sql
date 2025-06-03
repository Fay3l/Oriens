CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY NOT NULL,
    firstname VARCHAR(255) NOT NULL,
    lastname VARCHAR(255) NOT NULL,
    username VARCHAR(30) NOT NULL,
    address VARCHAR(255),
    email VARCHAR(255) NOT NULL UNIQUE,
    city VARCHAR(255),
    postalcode INTEGER,
    number_phone VARCHAR(20),
    age INTEGER NOT NULL,
    password VARCHAR(255) NOT NULL,
    role VARCHAR(255) NOT NULL,
    experience INTEGER DEFAULT 0 NOT NULL,
    badges JSONB DEFAULT '[]' NOT NULL,
    favorites_jobs VARCHAR(10),
    CONSTRAINT fk_jobs FOREIGN KEY(favorites_jobs) REFERENCES jobs(id)
);