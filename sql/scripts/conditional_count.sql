SELECT
    EXTRACT(MONTH FROM payment_date) AS "month",
    COUNT(*) AS total_count,
    SUM(amount) AS total_amount,
    COUNT(*) FILTER (WHERE staff_id = 1) AS mike_count,
    SUM(amount) FILTER (WHERE staff_id = 1) AS mike_amount,
    COUNT(*) FILTER (WHERE staff_id = 2) AS jon_count,
    SUM(amount) FILTER (WHERE staff_id = 2) AS jon_amount
FROM payment
GROUP BY "month"
ORDER BY "month"
