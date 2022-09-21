SELECT
    d.race,
    COUNT(*) AS "count"
FROM demographics AS d
GROUP BY d.race
ORDER BY "count" DESC
