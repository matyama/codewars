SELECT d.*
FROM departments AS d
WHERE
    EXISTS (
        SELECT 1
        FROM sales AS s WHERE s.department_id = d.id AND s.price > 98.00
    )
