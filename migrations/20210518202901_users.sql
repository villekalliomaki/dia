CREATE TABLE users (
    id              uuid DEFAULT uuid_generate_v4(),
    created         TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    modified        TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    username        VARCHAR(20) NOT NULL UNIQUE,
    email           VARCHAR(100) UNIQUE,
    display_name    VARCHAR(50),
    password_hash   TEXT NOT NULL,
    groups          VARCHAR(10)[] NOT NULL DEFAULT array[]::varchar[],
    PRIMARY KEY (id)
);