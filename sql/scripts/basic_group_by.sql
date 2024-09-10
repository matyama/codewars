SELECT
    p.age,
    COUNT(*) AS people_count
FROM people AS p
GROUP BY p.age
