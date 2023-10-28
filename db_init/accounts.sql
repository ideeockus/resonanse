create table user_accounts (
    -- base data
    id SERIAL PRIMARY KEY,
    username varchar(255) NOT NULL,

    -- user data
    first_name varchar(255) NOT NULL,
    last_name varchar(255) NOT NULL,
    city varchar(255) NOT NULL,
    about varchar(1023) NOT NULL,

    headline varchar(255),
    goals varchar(255),
    interests varchar(255),
    language varchar(255),
    age int,
    education varchar(255),

    hobby varchar(255),
    music varchar(255),
    sport varchar(255),
    books varchar(255),
    food varchar(255),
    worldview varchar(255),
    alcohol varchar(255),

    -- contacts data
    email varchar(255),
    phone varchar(255),
    telegram varchar(255),
    instagram varchar(255),

    -- auth data
    password_hash varchar(1023)
);
