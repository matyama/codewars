from typing import Mapping, Sequence


def namelist(names: Sequence[Mapping[str, str]]) -> str:
    text = ', '.join(item['name'] for item in names[:-1])
    last = names[-1]['name'] if names else ''
    return f'{text} & {last}' if text else last
