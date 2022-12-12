from collections.abc import Sequence

import pytest

from codewars.five_figure_summary import StatisticalSummary, Summary


@pytest.mark.parametrize(
    "data,precision,expected",
    [
        (range(7), 2, (7, 0, 6, 1.5, 3, 4.5)),
        (
            [
                "into",
                "injected",
                65,
                10.847455892994475,
                79,
                32.69983014127792,
                12.002255641934298,
                "the",
                46.72420025312117,
                49.86098741235106,
                "Random",
                4.343800579921544,
                52,
                2.937624659207444,
                4,
                6.358221428273202,
                43.14471731665904,
                7.373297841829058,
                34.39810981307107,
                17.836078163380055,
            ],
            2,
            (16, 2.94, 79, 7.12, 25.27, 47.51),
        ),
    ],
)
def test_summary(
    data: Sequence[float], precision: int | None, expected: Summary
) -> None:
    assert StatisticalSummary(data).five_figure_summary(precision) == expected
