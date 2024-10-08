WITH actor_film AS (
    SELECT
        a.actor_id,
        fa.film_id,
        CONCAT(a.first_name, ' ', a.last_name) AS actor_name
    FROM actor AS a
    INNER JOIN film_actor AS fa ON a.actor_id = fa.actor_id
),

top_pair AS (
    SELECT
        a1.actor_id AS fst_actor,
        a2.actor_id AS snd_actor
    FROM actor_film AS a1
    INNER JOIN
        actor_film AS a2
        ON
            a1.film_id = a2.film_id AND a1.actor_id < a2.actor_id
    GROUP BY fst_actor, snd_actor
    ORDER BY COUNT(*) DESC
    LIMIT 1
)

SELECT
    a1.actor_name AS first_actor,
    a2.actor_name AS second_actor,
    f.title
FROM top_pair AS p
INNER JOIN actor_film AS a1 ON p.fst_actor = a1.actor_id
INNER JOIN
    actor_film AS a2 ON p.snd_actor = a2.actor_id AND a1.film_id = a2.film_id
INNER JOIN film AS f ON a1.film_id = f.film_id
ORDER BY f.title
