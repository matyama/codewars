SELECT
    c.id AS category_id,
    c.category AS category,
    p.title AS title,
    p.views AS "views",
    p.id AS post_id
FROM categories AS c
LEFT JOIN LATERAL (
    SELECT *
    FROM posts AS p
    WHERE p.category_id = c.id -- noqa: L028
    ORDER BY p.views DESC, p.id ASC
    LIMIT 2
) AS p ON TRUE
ORDER BY category ASC, "views" DESC, post_id ASC
