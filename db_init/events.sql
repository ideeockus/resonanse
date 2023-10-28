create table resonanse_events (
    id UUID PRIMARY KEY,
    is_private BOOL NOT NULL,
    is_commercial BOOL NOT NULL,
    title varchar(255) NOT NULL,
    description varchar(1023) NOT NULL,
    subject INT NOT NULL,
    datetime TIMESTAMP NOT NULL,
--    timezone SMALLINT,
    location_latitude FLOAT8 NOT NULL,
    location_longitude FLOAT8 NOT NULL,
    location_title varchar(255),
    creator_id BIGINT NOT NULL REFERENCES user_accounts (id),
    event_type INT NOT NULL,
    picture UUID,
    creation_time TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

