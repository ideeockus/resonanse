create table resonanse_events (
    id UUID PRIMARY KEY,
    is_private BOOL NOT NULL,
    is_commercial BOOL NOT NULL,
    title varchar(255) NOT NULL,
    description varchar(1023) NOT NULL,
    subject varchar(255) NOT NULL,
    datetime TIMESTAMP NOT NULL,
    timezone SMALLINT,
    location_latitude float NOT NULL,
    location_longitude float NOT NULL,
    creator_id int REFERENCES user_accounts (id),
    event_type varchar(255) NOT NULL,
    picture varchar(1023),
    creation_time TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

