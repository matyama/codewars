SELECT
    e1.entry_time AS when_happened,
    COUNT(
        e1.id
    ) OVER (PARTITION BY e1.id ORDER BY e1.entry_time) AS visits_count
FROM visits AS e1
INNER JOIN
    visits AS e2 ON
        e2.entry_time <= e1.entry_time AND e1.entry_time < e2.exit_time
ORDER BY visits_count DESC
LIMIT 1
