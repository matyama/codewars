CREATE TYPE user_data AS (
    first_name text,
    last_name text,
    date_of_birth date,
    email_addresses jsonb,
    private boolean
);

SELECT
    u.first_name,
    u.last_name,
    EXTRACT(YEAR FROM AGE(u.date_of_birth)) AS age,
    CASE
        WHEN u.private THEN 'Hidden'
        ELSE COALESCE(u.email_addresses ->> 0, 'None')
    END AS email_address
FROM users, JSON_POPULATE_RECORD(NULL::user_data, users.data) AS u
ORDER BY u.first_name, u.last_name
