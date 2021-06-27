def rgb(r: int, g: int, b: int) -> str:
    return '%02X' * 3 % (clip(r), clip(g), clip(b))


def clip(v: int, low: int = 0, high: int = 255) -> int:
    return max(low, min(high, v))
