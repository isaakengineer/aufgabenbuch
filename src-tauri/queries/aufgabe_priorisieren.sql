UPDATE liste SET
	prioritaet = ?1,
	geaendert_an = datetime('now')
WHERE id = ?2
