from typing import List

VARS = {
    '0': ['0', '8'],
    '1': ['1', '2', '4'],
    '2': ['1', '2', '3', '5'],
    '3': ['2', '3', '6'],
    '4': ['1', '4', '5', '7'],
    '5': ['2', '4', '5', '6', '8'],
    '6': ['3', '5', '6', '9'],
    '7': ['4', '7', '8'],
    '8': ['0', '5', '7', '8', '9'],
    '9': ['6', '8', '9'],
}


def get_pins(observed: str) -> List[str]:
    pins = []
    pin = ['0'] * len(observed)

    def generate(i: int) -> None:
        if i == len(observed):
            pins.append(''.join(pin))
        else:
            for v in VARS[observed[i]]:
                pin[i] = v
                generate(i + 1)

    generate(0)
    return pins
