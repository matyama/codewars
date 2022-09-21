SELECT
    p.id,
    p.name,
    count(*) AS toy_count
FROM people AS p
INNER JOIN toys AS t ON p.id = t.people_id
GROUP BY p.id
