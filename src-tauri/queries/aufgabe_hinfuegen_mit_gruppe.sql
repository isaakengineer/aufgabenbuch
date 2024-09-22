INSERT INTO liste 
    (gruppe, beschreibung, wochentag, prioritaet, erstellt_an) 
    VALUES 
    (?1, ?2, ?3, ?4, datetime('now'));