UPDATE liste SET
    verschoben = ?1,
    getan = ?2,
    vernachlaessigt = ?3,
    kommentar = ?4,
    geaendert_an = datetime('now')
WHERE id = ?5
