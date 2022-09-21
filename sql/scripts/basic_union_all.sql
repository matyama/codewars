SELECT
    'US' AS "location",
    us.id AS "id",
    us.name,
    us.card_name,
    us.card_number,
    us.transaction_date
FROM ussales AS us
WHERE us.price > 50.00
UNION ALL
SELECT
    'EU' AS "location",
    eu.id AS "id",
    eu.name,
    eu.card_name,
    eu.card_number,
    eu.transaction_date
FROM eusales AS eu
WHERE eu.price > 50.00
ORDER BY "location" DESC, "id" ASC
