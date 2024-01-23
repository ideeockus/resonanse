create table bot_statistics (
    id BIGINT PRIMARY KEY,
    key varchar(255) UNIQUE NOT NULL,
    value varchar(255) NOT NULL
);

