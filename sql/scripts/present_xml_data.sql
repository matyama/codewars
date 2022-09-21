SELECT
    u.first_name,
    u.last_name,
    EXTRACT(YEAR FROM AGE(u.date_of_birth)) AS age,
    CASE
        WHEN u.private THEN 'Hidden'
        ELSE u.email_address
    END AS email_address
FROM users, XMLTABLE(
        '//data/user'
        passing data
        columns
        first_name text PATH 'first_name',
        last_name text PATH 'last_name',
        date_of_birth date PATH 'date_of_birth',
        email_address text PATH 'email_addresses/address[1]' DEFAULT 'None',
        private bool PATH 'private'
) AS u
ORDER BY u.first_name, u.last_name
