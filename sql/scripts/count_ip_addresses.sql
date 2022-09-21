SELECT ips.id, cnt.ips_between
FROM ip_addresses AS ips
LEFT JOIN LATERAL (
  SELECT ABS(
    SUM((x.a::bigint - x.b::bigint) * POWER(256, 3 - x.i + 1))::bigint
  ) AS ips_between
  FROM unnest(string_to_array(ips.first, '.'), string_to_array(ips.last, '.')) WITH ORDINALITY AS x(a, b, i)
) AS cnt ON TRUE
