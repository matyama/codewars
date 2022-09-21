SELECT
    p.name,
    COUNT(CASE d.detail WHEN 'good' THEN 1 END) AS good,
    COUNT(CASE d.detail WHEN 'ok' THEN 1 END) AS ok,
    COUNT(CASE d.detail WHEN 'bad' THEN 1 END) AS bad
FROM products AS p
INNER JOIN details AS d ON p.id = d.product_id
GROUP BY p.name
ORDER BY p.name
