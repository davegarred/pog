CREATE TABLE ff_teams
(
    owner      bigint NOT NULL,
    owner_name text   NOT NULL,
    team_name  text   NOT NULL,
    PRIMARY KEY (owner)
);

CREATE TABLE ff_attendance
(
    attendance_id int    NOT NULL,
    owner         bigint NOT NULL,
    week          int    NOT NULL,
    date          text   NOT NULL,
    PRIMARY KEY (attendance_id),
    FOREIGN KEY (owner) REFERENCES ff_teams ("owner")
);

CREATE SEQUENCE seq_ff_attendance_id OWNED BY ff_attendance.attendance_id;

INSERT INTO ff_teams(owner, owner_name, team_name)
VALUES (1048049562960539648, 'Steve', 'Big Baller Shot Caller'),
       (1046484657249718414, 'April', 'tacocat5'),
       (431634941626023936, 'Shawn', 'Rice-A-Roni'),
       (1045795671489380354, 'Anon', 'John Anon'),
       (1045764168210448384, 'Poca', 'Petey Sunshine'),
       (156425668270358529, 'Magnus', 'Crocks with Socks'),
       (1050119194533961860, 'Cisco', 'The Blumpkins'),
       (460972684986023937, 'Iris', 'Cryptids'),
       (1054147659289600060, 'Liz', 'Feelin'' the J Love'),
       (689977564202401792, 'Todd', 'Ayahuasca 8'),
       (885945439961108550, 'Lo', 'Uber FU'),
       (695398918694895710, 'Dave', 'Raider Nation');


INSERT INTO ff_attendance(attendance_id, owner, week, date)
VALUES (nextval('seq_ff_attendance_id'), 431634941626023936, 1, '2023-09-07'),
       (nextval('seq_ff_attendance_id'), 1045795671489380354, 1, '2023-09-07'),
       (nextval('seq_ff_attendance_id'), 1045764168210448384, 1, '2023-09-07'),
       (nextval('seq_ff_attendance_id'), 689977564202401792, 1, '2023-09-07'),
       (nextval('seq_ff_attendance_id'), 695398918694895710, 1, '2023-09-07'),

       (nextval('seq_ff_attendance_id'), 1048049562960539648, 1, '2023-09-10'),
       (nextval('seq_ff_attendance_id'), 431634941626023936, 1, '2023-09-10'),
       (nextval('seq_ff_attendance_id'), 695398918694895710, 1, '2023-09-10'),

       (nextval('seq_ff_attendance_id'), 1048049562960539648, 1, '2023-09-10'),
       (nextval('seq_ff_attendance_id'), 1046484657249718414, 1, '2023-09-10'),
       (nextval('seq_ff_attendance_id'), 431634941626023936, 1, '2023-09-10'),
       (nextval('seq_ff_attendance_id'), 1050119194533961860, 1, '2023-09-10'),
       (nextval('seq_ff_attendance_id'), 1054147659289600060, 1, '2023-09-10'),
       (nextval('seq_ff_attendance_id'), 695398918694895710, 1, '2023-09-10'),

       (nextval('seq_ff_attendance_id'), 1045795671489380354, 1, '2023-09-11'),
       (nextval('seq_ff_attendance_id'), 695398918694895710, 1, '2023-09-11'),

       (nextval('seq_ff_attendance_id'), 1045795671489380354, 2, '2023-09-17'),
       (nextval('seq_ff_attendance_id'), 1054147659289600060, 2, '2023-09-17'),

       (nextval('seq_ff_attendance_id'), 1048049562960539648, 3, '2023-09-24'),
       (nextval('seq_ff_attendance_id'), 431634941626023936, 3, '2023-09-24'),
       (nextval('seq_ff_attendance_id'), 1045795671489380354, 3, '2023-09-24'),
       (nextval('seq_ff_attendance_id'), 1045764168210448384, 3, '2023-09-24'),
       (nextval('seq_ff_attendance_id'), 156425668270358529, 3, '2023-09-24'),
       (nextval('seq_ff_attendance_id'), 1050119194533961860, 3, '2023-09-24'),
       (nextval('seq_ff_attendance_id'), 460972684986023937, 3, '2023-09-24'),
       (nextval('seq_ff_attendance_id'), 695398918694895710, 3, '2023-09-24'),

       (nextval('seq_ff_attendance_id'), 1048049562960539648, 3, '2023-09-24'),
       (nextval('seq_ff_attendance_id'), 431634941626023936, 3, '2023-09-24'),
       (nextval('seq_ff_attendance_id'), 1045795671489380354, 3, '2023-09-24'),
       (nextval('seq_ff_attendance_id'), 1045764168210448384, 3, '2023-09-24'),
       (nextval('seq_ff_attendance_id'), 460972684986023937, 3, '2023-09-24'),

       (nextval('seq_ff_attendance_id'), 1050119194533961860, 4, '2023-09-28'),
       (nextval('seq_ff_attendance_id'), 1054147659289600060, 4, '2023-09-28'),
       (nextval('seq_ff_attendance_id'), 695398918694895710, 4, '2023-09-28'),

       (nextval('seq_ff_attendance_id'), 1048049562960539648, 4, '2023-10-01'),
       (nextval('seq_ff_attendance_id'), 431634941626023936, 4, '2023-10-01'),
       (nextval('seq_ff_attendance_id'), 1045795671489380354, 4, '2023-10-01'),
       (nextval('seq_ff_attendance_id'), 156425668270358529, 4, '2023-10-01'),
       (nextval('seq_ff_attendance_id'), 1050119194533961860, 4, '2023-10-01'),
       (nextval('seq_ff_attendance_id'), 1054147659289600060, 4, '2023-10-01'),
       (nextval('seq_ff_attendance_id'), 695398918694895710, 4, '2023-10-01'),

       (nextval('seq_ff_attendance_id'), 1048049562960539648, 4, '2023-10-01'),
       (nextval('seq_ff_attendance_id'), 431634941626023936, 4, '2023-10-01'),
       (nextval('seq_ff_attendance_id'), 1045795671489380354, 4, '2023-10-01'),
       (nextval('seq_ff_attendance_id'), 156425668270358529, 4, '2023-10-01'),
       (nextval('seq_ff_attendance_id'), 1050119194533961860, 4, '2023-10-01'),
       (nextval('seq_ff_attendance_id'), 695398918694895710, 4, '2023-10-01'),

       (nextval('seq_ff_attendance_id'), 1048049562960539648, 4, '2023-10-02'),
       (nextval('seq_ff_attendance_id'), 1045795671489380354, 4, '2023-10-02'),
       (nextval('seq_ff_attendance_id'), 1050119194533961860, 4, '2023-10-02'),
       (nextval('seq_ff_attendance_id'), 695398918694895710, 4, '2023-10-02'),

       (nextval('seq_ff_attendance_id'), 431634941626023936, 5, '2023-10-08'),
       (nextval('seq_ff_attendance_id'), 1045764168210448384, 5, '2023-10-08'),
       (nextval('seq_ff_attendance_id'), 156425668270358529, 5, '2023-10-08'),
       (nextval('seq_ff_attendance_id'), 689977564202401792, 5, '2023-10-08'),
       (nextval('seq_ff_attendance_id'), 695398918694895710, 5, '2023-10-08'),

       (nextval('seq_ff_attendance_id'), 1046484657249718414, 5, '2023-10-09'),

       (nextval('seq_ff_attendance_id'), 1048049562960539648, 6, '2023-10-15'),
       (nextval('seq_ff_attendance_id'), 431634941626023936, 6, '2023-10-15'),
       (nextval('seq_ff_attendance_id'), 156425668270358529, 6, '2023-10-15'),
       (nextval('seq_ff_attendance_id'), 1050119194533961860, 6, '2023-10-15'),
       (nextval('seq_ff_attendance_id'), 689977564202401792, 6, '2023-10-15'),
       (nextval('seq_ff_attendance_id'), 695398918694895710, 6, '2023-10-15'),

       (nextval('seq_ff_attendance_id'), 1048049562960539648, 6, '2023-10-15'),
       (nextval('seq_ff_attendance_id'), 1046484657249718414, 6, '2023-10-15'),
       (nextval('seq_ff_attendance_id'), 431634941626023936, 6, '2023-10-15'),
       (nextval('seq_ff_attendance_id'), 156425668270358529, 6, '2023-10-15'),
       (nextval('seq_ff_attendance_id'), 1050119194533961860, 6, '2023-10-15'),
       (nextval('seq_ff_attendance_id'), 1054147659289600060, 6, '2023-10-15'),
       (nextval('seq_ff_attendance_id'), 689977564202401792, 6, '2023-10-15'),
       (nextval('seq_ff_attendance_id'), 695398918694895710, 6, '2023-10-15'),

       (nextval('seq_ff_attendance_id'), 431634941626023936, 6, '2023-10-16'),
       (nextval('seq_ff_attendance_id'), 1050119194533961860, 6, '2023-10-16'),
       (nextval('seq_ff_attendance_id'), 689977564202401792, 6, '2023-10-16'),
       (nextval('seq_ff_attendance_id'), 695398918694895710, 6, '2023-10-16'),

       (nextval('seq_ff_attendance_id'), 695398918694895710, 7, '2023-10-19'),

       (nextval('seq_ff_attendance_id'), 431634941626023936, 7, '2023-10-22'),
       (nextval('seq_ff_attendance_id'), 156425668270358529, 7, '2023-10-22'),
       (nextval('seq_ff_attendance_id'), 1054147659289600060, 7, '2023-10-22'),
       (nextval('seq_ff_attendance_id'), 695398918694895710, 7, '2023-10-22'),

       (nextval('seq_ff_attendance_id'), 431634941626023936, 7, '2023-10-22'),
       (nextval('seq_ff_attendance_id'), 156425668270358529, 7, '2023-10-22'),
       (nextval('seq_ff_attendance_id'), 1054147659289600060, 7, '2023-10-22'),
       (nextval('seq_ff_attendance_id'), 695398918694895710, 7, '2023-10-22'),

       (nextval('seq_ff_attendance_id'), 431634941626023936, 7, '2023-10-23'),
       (nextval('seq_ff_attendance_id'), 1050119194533961860, 7, '2023-10-23'),
       (nextval('seq_ff_attendance_id'), 695398918694895710, 7, '2023-10-23'),

       (nextval('seq_ff_attendance_id'), 1045764168210448384, 8, '2023-10-26'),
       (nextval('seq_ff_attendance_id'), 689977564202401792, 8, '2023-10-26'),
       (nextval('seq_ff_attendance_id'), 695398918694895710, 8, '2023-10-26'),

       (nextval('seq_ff_attendance_id'), 1048049562960539648, 8, '2023-10-29'),
       (nextval('seq_ff_attendance_id'), 1046484657249718414, 8, '2023-10-29'),
       (nextval('seq_ff_attendance_id'), 431634941626023936, 8, '2023-10-29'),

       (nextval('seq_ff_attendance_id'), 1048049562960539648, 8, '2023-10-29'),
       (nextval('seq_ff_attendance_id'), 1046484657249718414, 8, '2023-10-29'),
       (nextval('seq_ff_attendance_id'), 431634941626023936, 8, '2023-10-29'),
       (nextval('seq_ff_attendance_id'), 1050119194533961860, 8, '2023-10-29'),
       (nextval('seq_ff_attendance_id'), 695398918694895710, 8, '2023-10-29'),

       (nextval('seq_ff_attendance_id'), 1045764168210448384, 8, '2023-10-30'),
       (nextval('seq_ff_attendance_id'), 695398918694895710, 8, '2023-10-30'),

       (nextval('seq_ff_attendance_id'), 695398918694895710, 9, '2023-11-02'),

       (nextval('seq_ff_attendance_id'), 1048049562960539648, 9, '2023-11-05'),
       (nextval('seq_ff_attendance_id'), 431634941626023936, 9, '2023-11-05'),
       (nextval('seq_ff_attendance_id'), 1045764168210448384, 9, '2023-11-05'),
       (nextval('seq_ff_attendance_id'), 1054147659289600060, 9, '2023-11-05'),
       (nextval('seq_ff_attendance_id'), 695398918694895710, 9, '2023-11-05'),

       (nextval('seq_ff_attendance_id'), 1048049562960539648, 9, '2023-11-05'),
       (nextval('seq_ff_attendance_id'), 431634941626023936, 9, '2023-11-05'),
       (nextval('seq_ff_attendance_id'), 1045764168210448384, 9, '2023-11-05'),
       (nextval('seq_ff_attendance_id'), 1054147659289600060, 9, '2023-11-05'),
       (nextval('seq_ff_attendance_id'), 695398918694895710, 9, '2023-11-05'),

       (nextval('seq_ff_attendance_id'), 689977564202401792, 9, '2023-11-06'),
       (nextval('seq_ff_attendance_id'), 695398918694895710, 9, '2023-11-06'),

       (nextval('seq_ff_attendance_id'), 431634941626023936, 10, '2023-11-12'),
       (nextval('seq_ff_attendance_id'), 695398918694895710, 10, '2023-11-12'),

       (nextval('seq_ff_attendance_id'), 695398918694895710, 10, '2023-11-13'),

       (nextval('seq_ff_attendance_id'), 1050119194533961860, 11, '2023-11-16'),
       (nextval('seq_ff_attendance_id'), 689977564202401792, 11, '2023-11-16'),
       (nextval('seq_ff_attendance_id'), 695398918694895710, 11, '2023-11-16'),

       (nextval('seq_ff_attendance_id'), 1048049562960539648, 11, '2023-11-19'),
       (nextval('seq_ff_attendance_id'), 431634941626023936, 11, '2023-11-19'),
       (nextval('seq_ff_attendance_id'), 1050119194533961860, 11, '2023-11-19'),
       (nextval('seq_ff_attendance_id'), 460972684986023937, 11, '2023-11-19'),
       (nextval('seq_ff_attendance_id'), 1054147659289600060, 11, '2023-11-19'),
       (nextval('seq_ff_attendance_id'), 695398918694895710, 11, '2023-11-19'),

       (nextval('seq_ff_attendance_id'), 1048049562960539648, 11, '2023-11-19'),
       (nextval('seq_ff_attendance_id'), 431634941626023936, 11, '2023-11-19'),
       (nextval('seq_ff_attendance_id'), 156425668270358529, 11, '2023-11-19'),
       (nextval('seq_ff_attendance_id'), 1050119194533961860, 11, '2023-11-19'),
       (nextval('seq_ff_attendance_id'), 460972684986023937, 11, '2023-11-19'),
       (nextval('seq_ff_attendance_id'), 1054147659289600060, 11, '2023-11-19'),
       (nextval('seq_ff_attendance_id'), 689977564202401792, 11, '2023-11-19'),
       (nextval('seq_ff_attendance_id'), 695398918694895710, 11, '2023-11-19'),

       (nextval('seq_ff_attendance_id'), 695398918694895710, 11, '2023-11-20')
       ;

