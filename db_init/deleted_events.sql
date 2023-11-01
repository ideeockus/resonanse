create table deleted_events (
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
    updater_id BIGINT NOT NULL,
    event_type INT NOT NULL,
    picture UUID,
    contact_info varchar(255),
    creation_time TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    update_time TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

