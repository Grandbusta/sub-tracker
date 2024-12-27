-- Add migration script here
CREATE TABLE subscriptions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id),
    name VARCHAR NOT NULL,
    website_url VARCHAR NOT NULL,
    price INT NOT NULL,
    category VARCHAR NOT NULL,
    frequency TEXT NOT NULL,
    date_started TIMESTAMP NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);