-- Add up migration script here
-- this is where the code for the upslq will go dude 
CREATE TABLE users(
    email VARCHAR(75) NOT NULL PRIMARY KEY,
    hash VARCHAR(75) NOT NULL,
    created_at TIMESTAMP NOT NULL
);


