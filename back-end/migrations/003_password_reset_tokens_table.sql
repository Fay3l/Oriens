CREATE TABLE IF NOT EXISTS password_reset_tokens (
    user_id VARCHAR(36) NOT NULL,
    token VARCHAR(128) NOT NULL UNIQUE,
    token_expiry BIGINT NOT NULL,
    PRIMARY KEY (user_id, token)
);