UPDATE liste SET
    verschoben = NULL,
    getan = NULL,
    vernachlaessigt = NULL,
    kommentar = NULL,
    geaendert_an = datetime('now')
WHERE id = ?1
