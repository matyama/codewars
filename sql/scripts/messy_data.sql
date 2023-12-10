CREATE FUNCTION normalize(
    full_name text
) RETURNS text IMMUTABLE AS $$
DECLARE
  ms text[5];
  new_name text;
BEGIN
  ms := regexp_matches(full_name, '(([a-z]+\.|Miss)\s+)?([a-z\'']+)(,)?\s+([a-z\'']+).*', 'i');
  IF ms[4] IS NULL THEN
    new_name := CONCAT(ms[3], ' ', ms[5]);
  ELSE
    new_name := CONCAT(ms[5], ' ', ms[3]);
  END IF;
  RETURN lower(new_name);
END;
$$ LANGUAGE plpgsql;

SELECT
    c.first_name,
    c.last_name,
    c.credit_limit AS old_limit,
    p.credit_limit AS new_limit
FROM (
    SELECT
        *,
        normalize(concat(first_name, ' ', last_name)) AS norm_name
    FROM customers
) AS c
LEFT OUTER JOIN (
    SELECT
        normalize(p.full_name) AS norm_name,
        max(p.credit_limit) AS credit_limit
    FROM prospects AS p
    GROUP BY norm_name
) AS p ON c.norm_name = p.norm_name
WHERE p.credit_limit > c.credit_limit
