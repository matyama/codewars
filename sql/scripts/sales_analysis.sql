SELECT
    p.name AS product_name,
    EXTRACT(YEAR FROM s.date) AS "year",
    EXTRACT(MONTH FROM s.date) AS "month",
    EXTRACT(DAY FROM s.date) AS "day",
    SUM(p.price * sd.count) AS total
FROM sales_details AS sd
INNER JOIN products AS p ON sd.product_id = p.id
INNER JOIN sales AS s ON sd.sale_id = s.id
GROUP BY product_name, ROLLUP("year", "month", s.date)
ORDER BY product_name, "year", "month", "day"
