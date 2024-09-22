INSERT INTO liste 
    (beschreibung, wochentag, prioritaet, erstellt_an) 
    VALUES 
    (?1, ?2, ?3, datetime('now'));