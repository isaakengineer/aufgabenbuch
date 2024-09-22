INSERT INTO liste 
    (gruppe, beschreibung, wochentag, prioritaet, erstellt_an) 
    VALUES 
    (?1, ?2, 0, 0, datetime('now'));