SELECT
    p.pokemon_name,
    m.element,
    p.str * m.multiplier AS modifiedstrength
FROM pokemon AS p
INNER JOIN multipliers AS m ON p.element_id = m.id
WHERE p.str * m.multiplier >= 40
ORDER BY modifiedstrength DESC
