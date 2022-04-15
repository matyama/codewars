def is_prime(num: int) -> bool:
    if num <= 1:
        return False

    if num % 2 == 0:
        return num == 2

    n = 3

    while n**2 <= num:
        if num % n == 0:
            return False
        n += 1

    return True
