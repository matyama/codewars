import statistics as stats
from collections.abc import Sequence


def clean_mean(sample: Sequence[float], cutoff: int) -> float:
    """
    Compute the mean of given sample stripped from outliers.

    A point is considered to be an outlier if it's more than `cutoff` standard
    deviations from the sample mean.
    """
    outliers = True

    while outliers:
        mu = stats.mean(sample)
        std = stats.pstdev(sample, mu)
        new_sample = [x for x in sample if abs(x - mu) < cutoff * std]
        outliers = len(new_sample) < len(sample)
        sample = new_sample

    return round(mu, 2)
