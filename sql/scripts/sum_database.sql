CREATE OR REPLACE FUNCTION sum_db() RETURNS integer IMMUTABLE AS $$
DECLARE 
  total int := 0;
  total_col int;
  r RECORD;
BEGIN
  FOR r IN 
    SELECT
      t.schemaname AS schema_name,
      t.tablename AS table_name,
      c.column_name AS column_name
    FROM pg_catalog.pg_tables AS t
    JOIN (
     SELECT table_schema, table_name, column_name
      FROM information_schema.columns
      WHERE data_type = 'integer'
    ) AS c ON c.table_schema = t.schemaname AND c.table_name = t.tablename
    WHERE t.schemaname NOT IN ('pg_catalog', 'information_schema')
  LOOP
    EXECUTE format(
      'SELECT SUM(%I) AS total_col FROM %I.%I',
      r.column_name,
      r.schema_name,
      r.table_name
    ) INTO STRICT total_col;
    IF total_col IS NOT NULL THEN
      total := total + total_col;
    END IF;
  END LOOP;
  RETURN total;
END;
$$ LANGUAGE plpgsql; -- noqa: L016

SELECT sum_db() AS total;
