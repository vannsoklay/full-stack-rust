-- Add up migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE
    IF NOT EXISTS users (
        id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
        name VARCHAR(100) NOT NULL,
        email VARCHAR(255) NOT NULL UNIQUE,
        gender TEXT NOT NULL DEFAULT 'other',
        photo VARCHAR NOT NULL DEFAULT 'default.png',
        phone_number VARCHAR(20) NOT NULL,
        password VARCHAR(122) NOT NULL,
        verified BOOLEAN NOT NULL DEFAULT FALSE,
        role VARCHAR(50) NOT NULL DEFAULT 'user',
        created_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT NOW(),
            updated_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT NOW()
    );
CREATE INDEX users_email_idx ON users (email);

CREATE TABLE
    IF NOT EXISTS stories (
        id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
        user_id UUID NOT NULL,
        content TEXT NOT NULL,
        story_image VARCHAR NOT NULL DEFAULT 'story-default.png',
        published BOOLEAN DEFAULT FALSE,
        FOREIGN KEY (user_id) REFERENCES users(id) ON UPDATE CASCADE,
        created_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT NOW(),
            updated_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT NOW()
    );