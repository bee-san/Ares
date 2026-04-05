# Sum-O-Primes

## Challenge

We have so much faith in RSA we give you not just the product of the primes, but their sum as well!

* [gen.py](gen.py)
* [output.txt](output.txt)

## Solution

Run the solution [script.py](script.py), which uses the equations for `p` and `p-q` from [this paper](https://www.degruyter.com/document/doi/10.1515/jmc-2016-0046/html) ([Archive](https://web.archive.org/web/20220321005215/https://www.degruyter.com/document/doi/10.1515/jmc-2016-0046/html)).

The two equations can be combined to form: `p=(x+sqrt(x^2-4*n))/2`.

`gmpy2` is used to store large numbers and it's `precision` is set to `2048` ate the beginning of the script to ensure that no digits are lost.

### Flag

`picoCTF{3921def5}`
