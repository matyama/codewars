CREATE FUNCTION vector_add(a int[], b int[]) RETURNS int[] IMMUTABLE AS $$
  SELECT array_remove(ARRAY_AGG(i.a + i.b), NULL)
  FROM (SELECT * FROM unnest(a, b) UNION ALL SELECT NULL, NULL) AS i(a, b)
$$ LANGUAGE SQL;

CREATE OPERATOR + (
  leftarg = int[],
  rightarg = int[],
  function = vector_add,
  commutator = +
);
