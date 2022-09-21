WITH d AS (
    SELECT
        *,
        PCTINCREASE(department_id) AS increase
    FROM departments
)

SELECT
    e.employee_id,
    e.first_name,
    e.last_name,
    d.department_name,
    e.salary AS old_salary,
    e.salary * (1 + d.increase) AS new_salary
FROM employees AS e
INNER JOIN d ON e.department_id = d.department_id
ORDER BY e.employee_id;
