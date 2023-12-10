SELECT
    clans.*,
    RANK() OVER (ORDER BY clans.total_points DESC) AS rank
FROM (
    SELECT
        COALESCE(NULLIF(p.clan, ''), '[no clan specified]') AS clan,
        SUM(p.points) AS total_points,
        COUNT(*) AS total_people
    FROM people AS p
    GROUP BY p.clan
) AS clans
