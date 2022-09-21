WITH counts AS (
    SELECT
        DATE(created_at) AS "date",
        COUNT(*) AS "count"
    FROM posts
    GROUP BY date
)

SELECT
    *,
    (
        SUM(
            count
        ) OVER (ORDER BY date ROWS BETWEEN UNBOUNDED PRECEDING AND CURRENT ROW)
    )::int AS total
FROM counts
ORDER BY date
