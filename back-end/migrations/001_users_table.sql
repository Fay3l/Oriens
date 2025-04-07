CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(255) NOT NULL UNIQUE,
    firstname VARCHAR(255) NOT NULL,
    lastname VARCHAR(255) NOT NULL,
    address VARCHAR(255),
    email VARCHAR(255) NOT NULL UNIQUE,
    city VARCHAR(255),
    postalcode INTEGER,
    number_phone VARCHAR(15),
    age INTEGER,
    password VARCHAR(255) NOT NULL,
    experience INTEGER DEFAULT 0,
    badges JSONB DEFAULT '[]'
);