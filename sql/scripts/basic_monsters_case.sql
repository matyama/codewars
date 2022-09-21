SELECT
    t.id,
    t.heads,
    b.legs,
    t.arms,
    b.tails,
    CASE
        WHEN t.heads > t.arms OR b.tails > b.legs THEN 'BEAST'
        ELSE 'WEIRDO'
    END AS species
FROM top_half AS t
NATURAL JOIN bottom_half AS b
ORDER BY species
