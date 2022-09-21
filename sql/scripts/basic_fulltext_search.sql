SELECT *
FROM product
WHERE to_tsvector('english', name) @@ to_tsquery('english', 'Awesome')
