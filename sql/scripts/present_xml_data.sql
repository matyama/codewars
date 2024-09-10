SELECT
    u.first_name,
    u.last_name,
    EXTRACT(YEAR FROM AGE(u.date_of_birth)) AS age,
    CASE
        WHEN u.private THEN 'Hidden'
        ELSE u.email_address
    END AS email_address
FROM users,
    XMLTABLE(
        '//data/user'
        passing data --noqa: RF02
        columns --noqa: RF02
        first_name text PATH 'first_name', --noqa: RF02
        last_name text PATH 'last_name', --noqa: RF02
        date_of_birth date PATH 'date_of_birth', --noqa: RF02 
        email_address text --noqa: RF02
        PATH 'email_addresses/address[1]' DEFAULT 'None',
        private bool PATH 'private' --noqa: RF02
    ) AS u
ORDER BY u.first_name, u.last_name
