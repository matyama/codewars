def zeros(n: int) -> int:
    """
    Computes the number of trailing zeros in n!

    Algorithm taken from https://bit.ly/3mQeORK
    """
    cnt = 0

    while n >= 5:
        n //= 5
        cnt += n

    return cnt
