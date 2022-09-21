SELECT
    p.*,
    COUNT(*) AS sale_count,
    DENSE_RANK() OVER (ORDER BY COUNT(*) DESC) AS sale_rank
FROM people AS p
INNER JOIN sales AS s ON s.people_id = p.id
GROUP BY p.id
