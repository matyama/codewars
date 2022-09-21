SELECT
    m.id,
    m.name,
    POSITION(',' IN m.characteristics) AS comma
FROM monsters AS m
ORDER BY comma
