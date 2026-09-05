"""ReDoS regression for Network.Url.

The Python binding validates through the Rust core (the `regex` crate, an
RE2-family linear-time engine), so it was never vulnerable to the polynomial
backtracking CodeQL flagged in the JS RegExp path. This test guards that the
Network.Url pattern stays linear here too: a maliciously crafted input that made
the old JS pattern hang must reject essentially immediately.

The catastrophic input is "http://-.-" followed by many "--" and a trailing
non-URL char. The trailing char forces the match to fail; on a backtracking
engine the failure made it try every way to split the dashes between the host
repetition and the optional trailing group. The linear rewrite has exactly one
parse, so it rejects in constant-ish time.
"""

import time

from superscalar import SCALAR_ID_BY_CANONICAL, _native

import pytest

MALICIOUS = [
    "http://-.-" + "--" * 50000 + " ",
    "http://-.-" + "-" * 100000 + " ",
]


@pytest.mark.parametrize("inp", MALICIOUS)
def test_network_url_redos_rejects_fast(inp):
    sid = SCALAR_ID_BY_CANONICAL["Network.Url"]
    start = time.perf_counter()
    with pytest.raises(ValueError):
        _native.parse(sid, inp)
    elapsed_ms = (time.perf_counter() - start) * 1000
    assert elapsed_ms < 100, f"Network.Url validation took {elapsed_ms:.1f}ms (want < 100ms)"
