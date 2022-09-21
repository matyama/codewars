SELECT
    g.name,
    g.greeting,
    substring(g.greeting FROM '#(\d+)') AS user_id
FROM greetings AS g
