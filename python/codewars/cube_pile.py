def find_nb(m: int) -> int:

    n = 1
    total = 0

    while total < m:
        total += n**3
        n += 1

    return -1 if total > m else n - 1
