from collections import Counter
from typing import List, Sequence


def anagrams(word: str, words: Sequence[str]) -> List[str]:
    word_freq = Counter(word)
    return [w for w in words if Counter(w) == word_freq]
