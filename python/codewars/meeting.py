from typing import Tuple


def meeting(names: str) -> str:
    def parse_name(name: str) -> Tuple[str, str]:
        first_name, last_name = name.split(':')
        return last_name.upper(), first_name.upper()

    parsed_names = (parse_name(name) for name in names.split(';'))

    def name_str(name: Tuple[str, str]) -> str:
        last_name, first_name = name
        return f'({last_name}, {first_name})'

    return ''.join(name_str(name) for name in sorted(parsed_names))
