-- Add up migration script here
-- This is where the up data will go 
CREATE TABLE users(
    name VARCHAR(100) NOT NULL PRIMARY KEY,
    email VARCHAR(100) UNIQUE NOT NULL,
    pass VARCHAR(255) NOT NULL
);

