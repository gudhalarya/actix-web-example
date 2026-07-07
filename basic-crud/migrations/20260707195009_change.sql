-- Add migration script here
CREATE TABLE books (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    author TEXT NOT NULL,
    date TEXT NOT NULL,
    CONSTRAINT unique_book_author UNIQUE (name, author)
);