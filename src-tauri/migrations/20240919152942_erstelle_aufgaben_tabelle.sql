-- Add migration script here
CREATE TABLE IF NOT EXISTS liste (
    id INTEGER PRIMARY KEY,
    gruppe TEXT,
    beschreibung TEXT,
    notiz TEXT,
    link TEXT,
    wochentag INTEGER,
    prioritaet INTEGER,
    position INTEGER,
    verschoben DATE,
    getan DATE,
    vernachlaessigt DATE,
    kommentar TEXT,
    erstellt_an DATE,
    geaendert_an DATE
);