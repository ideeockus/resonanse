create table event_tg_table (
    post_id BIGINT PRIMARY KEY,
    event_id UUID NOT NULL REFERENCES resonanse_events (id)
);

