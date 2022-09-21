SELECT
    s.id,
    s.price,
    s.card_number,
    s.transaction_date,
    COALESCE(NULLIF(s.name, ''), '[product name not found]') AS "name",
    COALESCE(NULLIF(s.card_name, ''), '[card name not found]') AS card_name
FROM eusales AS s
WHERE s.price > 50
