SELECT INITCAP(CONCAT(firstname, ' ', lastname)) AS shortlist
FROM elves
WHERE firstname LIKE '%tegil%' OR lastname LIKE '%astar%'
