-- Add up migration script here
BEGIN; 
CREATE SCHEMA IF NOT EXISTS mainframe AUTHORIZATION CURRENT_USER;

CREATE TABLE IF NOT EXISTS mainframe.accounts (
    account_id SERIAL PRIMARY KEY,
    public_id uuid DEFAULT gen_random_uuid() NOT NULL UNIQUE,
    balance money NOT NULL
);
END;
