UPDATE liste SET 
    gruppe = ?1,
    beschreibung = ?2,
    notiz = ?3,
    link = ?4,
    wochentag = ?5, 
    prioritaet = ?6, 
    geaendert_an = datetime('now')
WHERE id = ?7;