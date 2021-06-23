CREATE TABLE IF NOT EXISTS refresh_tokens (
    id                  uuid DEFAULT uuid_generate_v4(),
    token_string        VARCHAR(100) NOT NULL,
    created             TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    modified            TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires             TIMESTAMPTZ NOT NULL CHECK(expires > created),
    user_id             uuid NOT NULL,
    client_address      VARCHAR(100) NOT NULL,
    max_jwt_lifetime    INT NOT NULL,
    CONSTRAINT refresh_tokens_pk
        PRIMARY KEY (id, token_string),
    CONSTRAINT refresh_token_user
        FOREIGN KEY(user_id)
            REFERENCES users(id) ON DELETE CASCADE
);