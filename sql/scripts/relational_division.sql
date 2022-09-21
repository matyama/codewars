SELECT f.title
FROM film AS f
INNER JOIN film_actor AS a ON a.film_id = f.film_id
WHERE a.actor_id IN (105, 122)
GROUP BY f.film_id
HAVING COUNT(*) = 2
ORDER BY f.title
