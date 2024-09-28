INSERT INTO liste
    (gruppe, beschreibung, wochentag, prioritaet, link, notiz, erstellt_an)
    VALUES
    (?1, ?2, ?3, ?4, ?5, ?6, datetime('now'));
