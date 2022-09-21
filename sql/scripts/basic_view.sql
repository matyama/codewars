CREATE VIEW members_approved_for_voucher AS
SELECT
    m.id,
    m.name,
    m.email,
    SUM(p.price) AS total_spending
FROM members AS m
INNER JOIN sales AS s ON s.member_id = m.id
INNER JOIN products AS p ON s.product_id = p.id
WHERE s.department_id IN (
    SELECT s.department_id
    FROM sales AS s
    INNER JOIN products AS p ON s.product_id = p.id
    GROUP BY s.department_id
    HAVING SUM(p.price) > 10000
    )
GROUP BY m.id
HAVING SUM(p.price) > 1000
ORDER BY m.id;

SELECT *
FROM members_approved_for_voucher
