SELECT
    p.ns[1] AS name,
    p.ns[2] AS first_lastname,
    p.ns[3] AS second_lastname
FROM (
    SELECT
        CASE
            WHEN array_length(a, 1) > 4 AND a[1] ILIKE 'The' THEN
              ARRAY[array_to_string(a[1:3], ' '), a[4], array_to_string(a[5:], ' ')]
            WHEN array_length(a, 1) > 3 THEN
              ARRAY[array_to_string(a[1:2], ' '), a[3], array_to_string(a[4:], ' ')]
            ELSE ARRAY[a[1], a[2], array_to_string(a[3:], ' ')]
        END AS ns
    FROM people, regexp_split_to_array(people.name, '\s+') AS a
) AS p
