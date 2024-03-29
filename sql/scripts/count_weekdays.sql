CREATE FUNCTION weekdays(d1 DATE, d2 DATE) RETURNS INTEGER
RETURNS NULL ON NULL INPUT
IMMUTABLE
AS
$$
  SELECT COUNT(*)
  FROM generate_series(LEAST(d1, d2), GREATEST(d1, d2), '1 day'::interval) AS d
  WHERE EXTRACT(isodow FROM d) < 6;
$$ LANGUAGE SQL;
