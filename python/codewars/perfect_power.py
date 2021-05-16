from typing import List, Optional


def exp(m: int, k: int) -> int:
    """Fast exponentiation: m ** k"""
    power = 1
    while k:
        if k & 1:
            power *= m
        m **= 2
        k >>= 1
    return power


# pylint: disable=invalid-name
def isPP(n: int) -> Optional[List[int]]:

    log2_n = (len(bin(n)) - 2) + 1

    # Search over k: k < log(n) + 1
    #  - TODO: It is sufficient to check only primes
    for k in range(2, log2_n):

        # Binary Search for m: m^k = n
        #  - i.e. k*log(m) = log(n) => log(m) = log(n) / k

        low_m = 1
        high_m = 1 << (log2_n // k + 1)

        while low_m < high_m - 1:

            m = (low_m + high_m) >> 1
            m_pow_k = exp(m, k)

            if m_pow_k > n:
                high_m = m
            elif m_pow_k < n:
                low_m = m
            else:
                return [m, k]

    return None
