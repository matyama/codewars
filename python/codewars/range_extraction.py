from typing import Iterable, List, Sequence


def solution(args: Sequence[int]) -> str:
    def output(buffer: Sequence[int]) -> Iterable[str]:
        if len(buffer) < 3:
            yield from map(str, buffer)
        else:
            yield f'{buffer[0]}-{buffer[-1]}'

    def ranges(xs: Sequence[int]) -> Iterable[str]:
        buffer: List[int] = []
        for x in xs:
            if buffer and x > buffer[-1] + 1:
                yield from output(buffer)
                buffer = [x]
            else:
                buffer.append(x)
        yield from output(buffer)

    return ','.join(ranges(args))
