WITH special_sales AS (
    SELECT *
    FROM sales
    WHERE price > 90.00
)

SELECT *
FROM departments
WHERE id IN (SELECT department_id FROM special_sales)
