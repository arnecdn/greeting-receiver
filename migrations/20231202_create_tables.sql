CREATE TABLE IF NOT EXISTS greeting
(
    id         NUMBER,
    from       VARCHAR(255) NOT NULL,
    to          VARCHAR(255) NOT NULL,
    heading     VARCHAR(255) NOT NULL,
    message      VARCHAR(255) NOT NULL,
    created     timestamp
);