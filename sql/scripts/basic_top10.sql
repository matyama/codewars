SELECT
    c.customer_id,
    c.email,
    SUM(p.amount)::float AS total_amount,
    COUNT(*) AS payments_count
FROM customer AS c
INNER JOIN payment AS p ON c.customer_id = p.customer_id
GROUP BY c.customer_id
ORDER BY total_amount DESC
LIMIT 10
