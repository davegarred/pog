CREATE TABLE wagers
(
    wager_id  int      NOT NULL,
    offering  text     NOT NULL,
    accepting text     NOT NULL,
    wager     text     NOT NULL,
    outcome   text     NOT NULL,
    status    smallint NOT NULL,
    PRIMARY KEY (wager_id)
);
CREATE SEQUENCE seq_wager_id MINVALUE 101 OWNED BY wagers.wager_id;

CREATE INDEX idx_wagers_offering ON wagers (offering);
CREATE INDEX idx_wagers_accepting ON wagers (accepting);
