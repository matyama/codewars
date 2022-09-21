WITH monthly_counts AS (
    SELECT
        date_trunc('month', p.created_at)::date AS "date",
        count(*) AS count,
        lag(
            count(*)
        ) OVER (ORDER BY date_trunc('month', p.created_at)::date) AS prev_count
    FROM posts AS p
    GROUP BY "date"
    ORDER BY "date"
)

SELECT
    c.date,
    c.count,
    round(
        (c.count - c.prev_count)::numeric / c.prev_count * 100, 1
    )::text || '%' AS percent_growth
FROM monthly_counts AS c
