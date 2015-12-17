#!/usr/bin/env python

'''
This script solves the kSum problem with dynamic programming.

The kSum is a general version of the 3Sum, i.e. return true if k elements in
an array of integers exist such that their sum equals s.

The runtime is O(k * s * N). (Ignore s < 0 cases)
'''

def ksum(a, z, s):
    # Guards
    if z < 0 or len(a) < z:
        return False
    elif z == 0 and s == 0:
        return True
    elif z == 0 and s != 0:
        return False

    # Our algorithm does not work with negative sums and integers.
    # Let's shift everything by the smallest element.
    m = min(a)
    b = a if m > 0 else map(lambda x: x - m, a)
    n = s if m > 0 else s - z*m

    # Another sanity check
    if n < 0:
        return False

    # z = 1
    # Let's build a dict for all possible sums
    sums = { i: True for i in b }

    # z = z + 1
    # Update sums for every z following
    new_sums = dict()
    for _ in range(z-1):
        for i in range(n+1):
            # Is sum_i possible
            sum_i_possible = False
            for e in b:
                _sum = i - e
                if _sum in sums and sums[_sum]:
                    sum_i_possible = True
            new_sums[i] = sum_i_possible
        sums = new_sums
        new_sums = dict()

    return sums[n]

if __name__ == "__main__":
    assert ksum([1], 2, 3) == False
    assert ksum([1], -2, 3) == False
    assert ksum([1], 0, 0) == True
    assert ksum([1], 0, 1) == False
    assert ksum([-1, 3], 2, -3) == False

    a = [18, 11, 21, 28, 31, -44, 38, 40, 55, 60, 62]
    assert ksum(a, 1, -44) == True
    assert ksum(a, 2, 67) == False
    assert ksum(a, 2, 66) == True
    assert ksum(a, 3, -15) == True
    assert ksum(a, 2, -15) == False
    assert ksum(a, 4, 13) == True
    print "All checks passed."
