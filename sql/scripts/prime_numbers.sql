WITH nats AS (
    SELECT 2 AS val
    UNION ALL
    SELECT val FROM generate_series(3, 100, 2) AS n(val)
)

SELECT n.val AS prime
FROM nats AS n
WHERE NOT EXISTS (
        SELECT 1 FROM nats AS m
        WHERE n.val > m.val AND n.val % m.val = 0
)
