def rgb(r: int, g: int, b: int) -> str:
    clip = lambda v: max(0, min(255, v))  # noqa: E731
    return '%02X' * 3 % (clip(r), clip(g), clip(b))
