CREATE TABLE user_session(
    session_key UUID PRIMARY KEY ,
    user_id SERIAL REFERENCES account(id) NOT NULL,
    is_valid BOOL NOT NULL,
    creation_timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
)