WITH id_matrices AS (
  SELECT ROW_NUMBER() OVER () AS id, *
  FROM matrices
), elements AS (
  SELECT m.id, m.matrix[i][j] AS x, j AS row
  FROM
    id_matrices AS m,
    generate_subscripts(m.matrix, 1) AS s1(i),
    generate_subscripts(m.matrix, 2) AS s2(j)
), rows AS (
  SELECT e.id, ARRAY_AGG(e.x) AS row
  FROM elements AS e
  GROUP BY e.id, e.row
  ORDER BY e.id, e.row
)
SELECT ARRAY_AGG(r.row) AS matrix
FROM rows AS r
GROUP BY r.id
ORDER BY r.id
