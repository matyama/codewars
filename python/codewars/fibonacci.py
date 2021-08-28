def fib(n: int) -> int:
    """Calculates the nth Fibonacci number in O(n) time"""

    if n < 0:
        fib_neg = fib(-n)
        return fib_neg if (1 - n) % 2 == 0 else -fib_neg

    fib_nn, fib_n = 0, 1

    for _ in range(2, n + 1):
        tmp = fib_n
        fib_n += fib_nn
        fib_nn = tmp

    return fib_n if n > 0 else fib_nn
