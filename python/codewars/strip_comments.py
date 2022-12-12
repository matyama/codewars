from collections.abc import Sequence


def strip_comments(strng: str, markers: Sequence[str]) -> str:
    def strip_line_comment(line: str) -> str:
        # use `str.find` in case markers consist of multiple characters
        hits = (i for m in markers if (i := line.find(m, 0)) >= 0)
        hit = min(hits, default=None)
        return line[:hit].rstrip() if hit is not None else line

    return "\n".join(map(strip_line_comment, strng.splitlines()))
