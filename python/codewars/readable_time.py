def make_readable(seconds: int) -> str:
    h = seconds // 3600
    m = (seconds - (h * 3600)) // 60
    s = seconds - (h * 3600) - (m * 60)
    return f"{h:02d}:{m:02d}:{s:02d}"
