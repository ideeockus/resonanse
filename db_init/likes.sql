create table user_likes (
    user_id BIGINT NOT NULL REFERENCES user_accounts (id),
    event_id UUID NOT NULL REFERENCES resonanse_events (id),
    event_score INT NOT NULL,
    PRIMARY KEY (user_id, event_id)
);
