CREATE TABLE admin_settings
(
    assignment text NOT NULL,
    settings   json NOT NULL,
    PRIMARY KEY (assignment)
);

INSERT INTO admin_settings(assignment, settings)
VALUES ('default', '{"welcome_channel": "","ff_year": 2024,"ff_week": 18}');


CREATE TABLE whois
(
    discord_id bigint NOT NULL,
    human_name text NOT NULL,
    hash_name  text NOT NULL,
    PRIMARY KEY (discord_id)
);




