SELECT capital
FROM countries
WHERE continent SIMILAR TO 'Afri(c|k)a' AND country LIKE 'E%'
ORDER BY capital
LIMIT 3
