CREATE EXTENSION tablefunc;

SELECT *
FROM crosstab(
    'SELECT p.name, d.detail, COUNT(*)
   FROM products AS p
   JOIN details AS d ON d.product_id = p.id
   GROUP BY p.name, d.detail
   ORDER BY p.name',
    $$ VALUES ('good'), ('ok'), ('bad') $$
)
