WITH RECURSIVE fib(fn_prev, fn) AS (
    SELECT
        0::bigint AS fn_prev,
        1::bigint AS fn

    UNION ALL

    SELECT
        fn AS fn_prev,
        fn_prev + fn AS fn
    FROM fib
)

SELECT fn_prev AS number
FROM fib
LIMIT 90
