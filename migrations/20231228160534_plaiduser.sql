-- Add migration script here
CREATE TABLE plaid (
    id VARCHAR(256) PRIMARY KEY,
    access_token VARCHAR(528) UNIQUE DEFAULT NULL,
    client_user_id VARCHAR(528) UNIQUE NOT NULL,
    item_id      VARCHAR(528) UNIQUE NOT NULL,
    phone_number VARCHAR(50) UNIQUE NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(), 
)
