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

INSERT INTO whois(discord_id, human_name, hash_name)
VALUES (1048049562960539648, 'Steve', 'No Titter'),
       (1046484657249718414, 'April', ''),
       (431634941626023936, 'Shawn', 'Zippercised'),
       (1045795671489380354, 'Craig', 'Emaculate Erection'),
       (1045764168210448384, 'Poca', 'Pocahomo'),
       (156425668270358529, 'Magnus', ''),
       (1050119194533961860, 'Cisco', ''),
       (460972684986023937, 'Iris', 'One Eyed, One horned...'),
       (1054147659289600060, 'Liz', ''),
       (689977564202401792, 'Todd', 'Abandon Bitch'),
       (885945439961108550, 'Lo', 'Uber'),
       (695398918694895710, 'Dave', 'FBS'),
       (1051369870241046598, 'Melissa', 'Pro Bone Her'),
       (1118245278538022952, 'Jason', ''),
       (458090379695095808, 'Sarah', 'My Little Penis'),
       (1052104625995649044, 'Molly', 'Shits twice and gags'),
       (520458944020480003, '', 'Hello Kitty'),
       (758467043795795999, 'Becky', ''),
       (348912925609820162, 'Josh', 'Dick in a Box'),
       (770010701758070824, 'Jeff', ''),
       (1056623969492545618, 'Lysne', ''),
       (1051372294922047488, 'Michelle', 'T-Ball'),
       (504066553331974175, 'Ashley', 'Titty Stardust'),
       (667543504666361896, 'Kat', 'Soup'),
       (745792133801181345, 'Inga', ''),
       (1056092093115809852, 'Megan', ''),
       (695112533698150510, 'Hilary', ''),
       (823054172524904459, 'Cassie', '');




