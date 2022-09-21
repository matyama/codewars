WITH history AS (
    SELECT
        id,
        value,
        LAG(id) OVER (ORDER BY id) AS prev_id,
        LAG(value) OVER (ORDER BY id) AS prev_value
    FROM entries
)

SELECT
    id,
    value,
    SUM(CASE WHEN id = prev_id + 1 AND value = prev_value THEN 0 ELSE 1 END)
    OVER (
        ORDER BY id ROWS BETWEEN UNBOUNDED PRECEDING AND CURRENT ROW
    ) AS run_id
FROM history
