SELECT
    d.name AS department,
    DATE(s.transaction_date) AS "day",
    COUNT(s.id) AS sale_count
FROM department AS d
INNER JOIN sale AS s ON d.id = s.department_id
GROUP BY "day", d.name
ORDER BY "day" ASC
