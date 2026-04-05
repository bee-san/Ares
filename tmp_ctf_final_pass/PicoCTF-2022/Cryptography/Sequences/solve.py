from sympy import Function, rsolve
from sympy.abc import n
from gmpy2 import mpz

####### Begin code from sequences.py
import hashlib
import sys
ITERS = int(2e7)
VERIF_KEY = "96cc5f3b460732b442814fd33cf8537c"
ENCRYPTED_FLAG = bytes.fromhex("42cbbce1487b443de1acf4834baed794f4bbd0dfb5885e6c7ed9a3c62b")

def decrypt_flag(sol):
    # sol = sol % (10**10000)
    sol = str(sol)
    sol_md5 = hashlib.md5(sol.encode()).hexdigest()

    if sol_md5 != VERIF_KEY:
        print("Incorrect solution")
        sys.exit(1)

    key = hashlib.sha256(sol.encode()).digest()
    flag = bytearray([char ^ key[i] for i, char in enumerate(ENCRYPTED_FLAG)]).decode()

    print(flag)
####### End code from sequences.py

# Define the function using sympy's symbols.
y = Function('y')
f = y(n+4) - 21 * y(n+3) - 301 * y(n+2) + 9549 * y(n+1) - 55692 * y(n)

# Solve the recurrence equation using `rsolve` with the given initial conditions.
equation = rsolve(f, y(n), {y(0): 1, y(1): 2, y(2): 3, y(3): 4})
print(equation)

# Rewrite the equation using gmpy2's `mpz` (arbitrary precision integers) for quick computations.
# See: https://gmpy2.readthedocs.io/en/latest/mpz.html. We have to do this manually because sympy's
# `lambdify` function does not support `gmpy2`. We also could use `evalf` by running
# `equation.evalf(precision, subs={n: 2e7}`, but this is meant for floating point values, not integers
# and thus would require us to know/guess a correct amount of precision.
equation = lambda n: 403*mpz(-21)**int(n)//10659 + 760*mpz(12)**int(n)//33 - 1727*mpz(13)**int(n)//68 + 253*mpz(17)**int(n)//76
solution = equation(n=2e7) % 10**10_000

print(solution)
decrypt_flag(solution+1)
