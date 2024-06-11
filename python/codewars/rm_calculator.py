def epley(w: float, r: int) -> float:
    return w * (1 + r / 30.0)


def mcglothin(w: float, r: int) -> float:
    return 100 * w / (101.3 - 2.67123 * r)


def lombardi(w: float, r: int) -> float:
    return w * float(r**0.10)


def calculate_1RM(w: float, r: int) -> int:
    match r:
        case 0:
            return 0
        case 1:
            return round(w)
        case _:
            return round(max(epley(w, r), mcglothin(w, r), lombardi(w, r)))
