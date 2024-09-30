UPDATE liste SET
  position = ?1,
  geaendert_an = datetime('now')
WHERE id = ?2
