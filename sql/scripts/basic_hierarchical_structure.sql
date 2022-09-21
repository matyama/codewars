WITH RECURSIVE employee_levels AS (
    SELECT
        1 AS "level",
        *
    FROM employees
    WHERE manager_id IS NULL

    UNION ALL

    SELECT
        el.level + 1 AS "level",
        e.*
    FROM employees AS e
    INNER JOIN employee_levels AS el ON e.manager_id = el.id
)

SELECT * FROM employee_levels;
