SELECT
    y.player_name,
    y.games,
    ROUND(
        y.hits::numeric / y.at_bats::numeric, 3
    )::text AS batting_average
FROM yankees AS y
WHERE y.at_bats >= 100
ORDER BY y.hits::numeric / y.at_bats::numeric DESC
