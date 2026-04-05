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


e = 0x10001
c = 0x19a98df2bfd703a31fedff8a02d43bc11f1fb3c15cfa7a55b6a32b3532e1ac477f6accc448f9b7d2b4deaae887450217bb70298afaa0f5e31a77e7c6f8ba1986979f15d299230119e3dd7e42eb9ca4d58d084d18b328fbe08c8909a2afc67866d6550e4e6fa27dc13d05c51cc87259fe73e2a1890cc2825d76c8b2a99f72f6023fc96658ac355487a6c275717ca6c13551094818efae1cec3c8773cc5a72fed518c00a53ba9799d9d5c182795dfcece07c727183fdd86fd2cb4b95e9f231be1858320aa7f8430885eb3d24300552d1a83158636316e55e6ac0a30a608964dbf2c412aed6a15df5fd49e737f7c06c02360d0c292abc33a3735152db2fb5bc5f6d
n = 0x65446ab139efe9744c78a271ad04d94ce541a299f9d4dcb658f66f49414fb913d8ac6c90dacc1ad43135454c3c5ac76c56d71d2816dac23db5c8caa773ae2397bd5909a1f2823c230f44ac684c437f16e4ca75d50b75d2f7e5549c034aa8a723c9eaa904572a8c5c6c1ed7093a0695522a5c41575c4dbf1158ca940c02b223f50ae86e6782819278d989200a2cd2be4b7b303dffd07209752ee5a3060c6d910a108444c7a769d003bf8976617b4459fdc15a2a73fc661564267f55be6a0d0d2ec4c06a4951df5a096b079d9e300f7ad72fa6c73a630f9a38e472563434c10225bde7d08c651bdd23fd471077d44c6aab4e01323ed78641983b29633ad104f3fd
num_primes = 7_000
p, q = pollard_P_1(n, num_primes=num_primes)
print(p)
print(q)
if q is None:
    print("Pollard p-1 Factorization Attack Failed. You can try increasing `num_primes`...")
else:
    n = p * q

    m = gmpy2.lcm(p - 1, q - 1)
    d = pow(e, -1, m)

    m = pow(c, d, n)
    print("Flag: %s" % binascii.unhexlify(hex(m)[2:]).decode())
