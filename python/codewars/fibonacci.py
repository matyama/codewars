def fib(n: int) -> int:
    """
    Calculates the n-th Fibonacci number in O(log(n)) time.
    See: [Exercise 1.19](https://bit.ly/3Bhv2JR)
    """
    if n < 0:
        fib_neg = fib(-n)
        return fib_neg if (1 - n) % 2 == 0 else -fib_neg

    a, b, p, q = 1, 0, 0, 1

    while n:
        if n % 2 == 0:
            p, q = (p ** 2 + q ** 2), (q ** 2 + 2 * p * q)
            n //= 2
        else:
            a, b = (b * q + a * p + a * q), (b * p + a * q)
            n -= 1

    return b
