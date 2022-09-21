SELECT
    r.category_id,
    r.category,
    r.title,
    r.views,
    r.post_id
FROM (
        SELECT
            c.id AS category_id,
            c.category,
            p.title,
            p.views,
            p.id AS post_id,
            DENSE_RANK() OVER w AS view_rank
        FROM posts AS p
        RIGHT OUTER JOIN categories AS c ON c.id = p.category_id
        WINDOW w AS (PARTITION BY c.id ORDER BY p.views DESC, p.id ASC)
) AS r
WHERE r.view_rank <= 2
ORDER BY r.category ASC, r.views DESC, r.post_id ASC
