CREATE TABLE channel_comments
(
    comment_id bigint    NOT NULL,
    user_id    bigint    NOT NULL,
    user_name  text      NOT NULL,
    time       timestamp NOT NULL,
    channel_id bigint    NOT NULL,
    comment    text      NOT NULL,
    PRIMARY KEY (comment_id)
);

CREATE INDEX idx_channel_comments_channel_id ON channel_comments (channel_id);

