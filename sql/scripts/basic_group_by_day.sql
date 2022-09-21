SELECT
    description,
    DATE(created_at) AS "day",
    COUNT(*) AS "count"
FROM events
WHERE name = 'trained'
GROUP BY "day", description
ORDER BY "day"
