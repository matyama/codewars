from collections import Counter, deque
from dataclasses import dataclass
from functools import cached_property
from itertools import starmap
from typing import Collection, Deque, Dict, List, Optional, Tuple, Union

Freqs = Collection[Tuple[str, int]]
Tree = Union["Leaf", "Node"]


@dataclass
class Leaf:
    weight: int
    symbol: str
    pred: Optional["Node"]

    @staticmethod
    def new(symbol: str, weight: int) -> "Leaf":
        return Leaf(weight=weight, symbol=symbol, pred=None)

    @cached_property
    def bits(self) -> str:
        bits = []
        node: Tree = self
        while node.pred is not None:
            n = node.pred
            bits.append(int(node is n.right))
            node = n

        return "".join(str(b) for b in reversed(bits))


@dataclass
class Node:
    weight: int
    left: Tree
    right: Tree
    pred: Optional["Node"]

    @staticmethod
    def new(left: Tree, right: Tree) -> "Node":
        node = Node(
            weight=left.weight + right.weight,
            left=left,
            right=right,
            pred=None,
        )
        left.pred = node
        right.pred = node
        return node


@dataclass(frozen=True)
class HuffmanTree:
    root: Node
    leafs: Dict[str, Leaf]

    @classmethod
    def build(cls, freqs: Freqs) -> Optional["HuffmanTree"]:
        freqs = sorted(freqs, key=lambda pair: pair[1])
        return cls.build_presorted(freqs)

    @classmethod
    def build_presorted(cls, freqs: Freqs) -> Optional["HuffmanTree"]:
        if len(freqs) < 2:
            return None

        leaf_refs: Dict[str, Leaf] = {}

        leafs = deque(starmap(Leaf.new, freqs), maxlen=len(freqs))
        nodes: Deque[Node] = deque(maxlen=len(leafs) // 2)

        def min_queue(
            lq: Deque[Leaf], nq: Deque[Node]
        ) -> Union[Deque[Leaf], Deque[Node]]:
            if not lq:
                return nq
            if not nq:
                return lq
            return lq if lq[0].weight <= nq[0].weight else nq

        while len(leafs) + len(nodes) > 1:
            left = min_queue(leafs, nodes).popleft()
            if isinstance(left, Leaf):
                leaf_refs[left.symbol] = left

            right = min_queue(leafs, nodes).popleft()
            if isinstance(right, Leaf):
                leaf_refs[right.symbol] = right

            nodes.append(Node.new(left, right))

        assert not leafs and len(nodes) == 1
        return cls(root=nodes[0], leafs=leaf_refs)

    def encode(self, symbol: str) -> str:
        return self.leafs[symbol].bits


def frequencies(s: str) -> Freqs:
    return Counter(s).items()


def encode(freqs: Freqs, s: str) -> Optional[str]:
    tree = HuffmanTree.build(freqs)

    if tree is None:
        return None

    return "".join(tree.encode(c) for c in s)


def decode(freqs: Freqs, bits: str) -> Optional[str]:
    tree = HuffmanTree.build(freqs)

    if tree is None:
        return None

    chars: List[str] = []
    node = tree.root

    for bit in bits:
        n = node.right if int(bit) else node.left

        if isinstance(n, Leaf):
            chars.append(n.symbol)
            node = tree.root
        else:
            node = n

    assert node is tree.root
    return "".join(chars)
