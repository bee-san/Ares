# Python code for Pollard p-1 Factorization Method
# Based on https://github.com/Ganapati/RsaCtfTool/blob/c13713a2808a03b15eb62e35605b9eb4271069cc/attacks/single_key/pollard_p_1.py

import binascii
import gmpy2
import math
from tqdm import tqdm


def _primes_yield_gmpy(n):
    p = i = 1
    while i <= n:
        p = gmpy2.next_prime(p)
        yield p
        i += 1


def primes(n):
    return list(_primes_yield_gmpy(n))


def pollard_P_1(n, progress=True, num_primes=2000):
    """Pollard P1 implementation"""
    z = []
    logn = math.log(int(gmpy2.isqrt(n)))
    prime = primes(num_primes)

    for j in range(0, len(prime)):
        primej = prime[j]
        logp = math.log(primej)
        for i in range(1, int(logn / logp) + 1):
            z.append(primej)

    try:
        for pp in tqdm(prime, disable=(not progress)):
            i = 0
            x = pp
            while 1:
                x = gmpy2.powmod(x, z[i], n)
                i = i + 1
                y = gmpy2.gcd(n, x - 1)
                if y != 1:
                    p = y
                    q = n // y
                    return p, q
                if i >= len(z):
                    return 0, None
    except TypeError:
        return 0, None


e = 3
c = 0x51099773fd2aafd5f84dfe649acbb3558797f58bdc643ac6ee6f0a6fa30031767966316201c36be69241d9d05d0bd181ced13809f57b0c0594f6b29ac74bc7906dae70a2808799feddc71cf5b28401100e5e7e0324b9d8b56e540c725fa4ef87b9e8d0f901630da5f7f181f6d5b4cdc00d5f5c3457674abcb0d0c173f381b92bdfb143c595f024b98b9900410d502c87dfc1633796d640cb5f780fa4b6f0414fb51e34700d9096caf07b36f4dcd3bb5a2d126f60d3a802959d6fadf18f4970756f3099e14fa6386513fb8e6cdda80fdc1c32a10f6cdb197857caf1d7abf3812e3d9dcda106fa87bac382d3e6fc216c55da02a0c45a482550acb2f58bea2cfa03
n = 0xd63c7cb032ae4d3a43ecec4999cfa8f8b49aa9c14374e60f3beeb437233e44f988a73101f9b20ffb56454350b1c9032c136142220ded059876ccfde992551db46c27f122cacdd38c86acb844032f8600515aa6ccb7a1d1ac62d04b51b752476d2d6ee9f22d0f933bebdd833a71fd30510479fcc7ba0afb1d4b0a1622cdc2a48341010dffdcfc8d9af45959fb30b692dc2c9e181ac6bcd6a701326e3707fb19b7f9dfe1c522c68f9b0d229d384be1e1c58f72f8df60ca5172a341a7ee81428a064beedd6af7b89cc6079f2b6d3717f0d29330f0a70acca05bf67ab60c2e5cb0b86bfca2c9b8d50d79d24371432a1efb243f3c5f15b377ccc51f6e69bfbf5ecc61
num_primes = 20_000

p, q = pollard_P_1(n, num_primes=num_primes)

print(f"p: {p:x}")
print(f"q: {q:x}")
if q is None:
    print("Pollard p-1 Factorization Attack Failed. You can try increasing `num_primes`...")
