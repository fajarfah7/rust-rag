-- Add up migration script here
DROP TABLE IF EXISTS public.users;
CREATE TABLE public.users(
    id UUID PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    username VARCHAR(100) UNIQUE NOT NULL,
    email VARCHAR(100) UNIQUE NOT NULL,
    phone_number VARCHAR(20) NULL,
    encrypted_password VARCHAR(255) NOT NULL,
    photo_profile VARCHAR(255) NULL,
    token_version INT DEFAULT 0
);