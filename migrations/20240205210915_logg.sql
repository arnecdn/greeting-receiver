-- Add migration script here
CREATE TABLE IF NOT EXISTS greeting_logg
(
    id              BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY ,
    greeting_id     UUID,
    created         TIMESTAMP NOT NULL
);