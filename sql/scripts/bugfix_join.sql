SELECT
    j.job_title,
    ROUND(AVG(j.salary), 2)::float AS average_salary,
    ROUND(SUM(j.salary), 2)::float AS total_salary,
    COUNT(p.id) AS total_people
FROM people AS p
INNER JOIN job AS j ON p.id = j.people_id
GROUP BY j.job_title
ORDER BY average_salary DESC
