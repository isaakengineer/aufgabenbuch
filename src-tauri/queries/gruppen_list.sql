SELECT
  gruppe AS `name`,
  COUNT(id) AS `count`
FROM
  liste
WHERE
  (vernachlaessigt IS NULL AND getan IS NULL AND verschoben IS NULL)
GROUP BY
  gruppe
