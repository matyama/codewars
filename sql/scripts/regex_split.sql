SELECT
    project,
    regexp_replace(address, '[\d]', '', 'g') AS letters,
    regexp_replace(address, '[^\d]', '', 'g') AS numbers
FROM repositories
