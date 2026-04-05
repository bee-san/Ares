# NSA Backdoor

## Challenge

I heard someone has been sneakily installing backdoors in open-source implementations of Diffie-Hellman... I wonder who it could be... ;)

* [gen.py](./gen.py)
* [output.txt](./output.txt)

## Solution

This challenge was very difficult for me and took over 10 hours. This is likely the intended method. Scroll down for a much simpler approach.

The [gen.py](./gen.py) script is very similar to the script provided in the [Very Smooth](../Very%20Smooth/README.md) challenge. The largest difference is that `c = pow(3, FLAG, n)` is used instead of `c = pow(FLAG, e, n)` (where `e = 3`). Even though only one value was swapped, this is a substantial change that changes the encryption algorithm from RSA to Diffie-Hellman. So, instead of solving the discrete d-th root problem (RSA), we are solving the discrete logarithm problem (Diffie-Hellman). [Here's a short explanation](https://crypto.stackexchange.com/a/803) of the major difference between RSA and Diffie-Hellman.

There is one hint for this challenge and it reads "Look for Mr. Wong's whitepaper... His work has helped so many cats!" Searching online for "Wong NSA Backdoor" finds [this paper titled "How to Backdoor Diffie-Hellman"](https://eprint.iacr.org/2016/644.pdf) ([Archive](https://web.archive.org/web/20220224174418/https://eprint.iacr.org/2016/644.pdf)). Searching for "diffie-hellman David Wong" finds [this article](https://www.cryptologie.net/article/376/defcon-how-to-backdoor-diffie-hellman/) (this is David Wong's website), which links to [this YouTube video](https://www.youtube.com/watch?v=90EYVy35gsY) of a talk by David Wong explaining his paper. The paper and talk both link to [mimoo/Diffie-Hellman_Backdoor](https://github.com/mimoo/Diffie-Hellman_Backdoor) on GitHub, which is a repo containing the code for this backdoor exploit.

To solve this challenge, I read the paper multiple times, watched the presentation, and looked at a variety of miscellaneous resources on the internet. So, I suggest you also read the paper and watch the presentation to gain a deep understanding of this challenge.

The paper works up to an attack called "Composite Modulus with B-Smooth Order (CM-HSO)," which is what we will be using in this challenge. In order for the CM-HSO backdoor to be a NOBUS (Nobody-But-Us) backdoor, the composite modulus $n = pq$ must be large enough such that it is not susceptible to factoring with [Pollard's p-1 algorithm](https://en.wikipedia.org/wiki/Pollard's_p_%E2%88%92_1_algorithm) (just like in RSA). To quote the paper, this method's "security also relies on the RSA’s assumption that factoring n is difficult if n is large enough." In the paper and code, "as a way of countering Pollard's p-1 we can add a large factor to both $p-1$ and $q-1$ that we will call $p_{big}$ and $q_{big}$ respectively." However, as we saw in [Very Smooth](../Very%20Smooth/README.md), the public modulus is factorable with Pollard's p-1 algorithm since these large factors are not used and because [smooth primes](https://en.wikipedia.org/wiki/Smooth_number#Powersmooth_numbers) are used in the [gen.py](./gen.py) script. Thus, we can use the same script as in [Very Smooth](../Very%20Smooth/README.md) to factor $n$. Running the [factor.py](./factor.py) script with the given `n` produces the following values for `p` and `q`:

```
p: ec4198b499d71ea60b224a4a9f0f04576fa8fd36485e05fd79a6ff1527be325a7a598341bbbedcd728b745525cc4b569f91a631ef74ee44f319e5f4d38bf3b9cb3d14b1a6e629553b831987695d0976a76a24860a23a7ebec42cbe41c625c8013e174ce1d19804e4b7111d8adab1a4690b5341c5897fcd33163077f07a4d0a17
q: e823cd272413ba5dbc8ade057120e2488345eea930e0b42f97d949c36e29218c2760059fef64d97da2a06144cb72e6451260d7e8f6d3cb78641131bdc2c8c09dc4f9395e0b1e9ac20d1266c9058b8c0e22ec7071236b1ab559188ed23de93213af1819453419f2108b453d3c9342e99a5a16e68acfe19b69af4b58b019a70047
```

Note that [ZeroBone/PollardRsaCracker](https://github.com/ZeroBone/PollardRsaCracker) is another implementation of Pollard's p-1 that you can try, but I found [RsaCtfTool's implementation](https://github.com/Ganapati/RsaCtfTool/blob/c13713a2808a03b15eb62e35605b9eb4271069cc/attacks/single_key/pollard_p_1.py
), which I use in [factor.py](./factor.py), to be better.

According to the paper, "Since $p-1$ and $q-1$ are both B-smooth, they are susceptible to be factored with the Pollard's p-1 factorization algorithm, a factorization algorithm that can find a factor $p$ of $n$ if $p-1$ is partially-smooth."

Next, we need to compute $x$ where $g^x\equiv c\pmod n$ (can also we written as $g^x\bmod n=c$ since $c<n$). We know all variables except $x$ and we know that $n=p*q$.

In the [backdoor_generator](https://github.com/mimoo/Diffie-Hellman_Backdoor/tree/master/backdoor_generator) folder of the [GitHub repo](https://github.com/mimoo/Diffie-Hellman_Backdoor) there is a [script to generate backdoored Diffie-Hellman parameters](https://github.com/mimoo/Diffie-Hellman_Backdoor/blob/master/backdoor_generator/backdoor_generator.sage) called `backdoor_generator.sage` and a [script to test the attacks](https://github.com/mimoo/Diffie-Hellman_Backdoor/blob/master/backdoor_generator/backdoor_generator_tests.sage) called `backdoor_generator_tests.sage`. The testing script sounds the most interesting because it shows off the exploit versus the generator script shows how to set up the exploit.

Note that before continuing you are going to want to have [SageMath](https://en.wikipedia.org/wiki/SageMath) installed on your computer, which you can do by following their [official installation guide](https://doc.sagemath.org/html/en/installation/).

The `backdoor_generator.sage` script discusses two methods of creating backdoored Diffie-Hellman values: Composite Modulus with Hidden Small Subgroup (CM-HSS) and CM-HSO, which I mentioned earlier. CM-HSS is what I thought was originally being used in this challenge since the paper states that it is no longer a Nobody-But-Us backdoor due to the work of [Dorey et al.](https://eprint.iacr.org/2016/999) ([Archive](https://web.archive.org/web/20220313071015/https://eprint.iacr.org/2016/999.pdf)) and [Coron et al's attack](https://eprint.iacr.org/2010/650) ([Archive](https://web.archive.org/web/20220120205927/https://eprint.iacr.org/2010/650.pdf)).

As we can see in the [`backdoor_generator_tests.sage` file](./backdoor_generator_tests.sage) (which I have saved to this directory) and section 4 of the paper, "two small subgroups" are used. These small subgroups exist because a generator g is chosen "such that both g modulo p and g modulo q generate 'small' subgroups." The existence of the subgroups allows "us to compute two discrete logarithms in two small subgroups instead of one discrete logarithm in one large group." The paper states "For example, we could pick $p$ and $q$ such that $p-1 = 2_{p_1p_2}$ and $q-1 = 2_{q_1q_2}$ with $p_1$ and $q_1$ two small prime factors and $p_2$, $q_2$ two large prime factors." Indeed, if we open [backdoor_generator_tests.sage](./backdoor_generator_tests.sage) and look at the examples at the start of the `test_CM_HSS` function we see a small factor for `p-1` and another small factor for `q-1`. We can confirm this by taking the bigger `p` value and getting prime factors of `p-1` in SageMath by running the below command:

```
prime_factors(7323720966914812591055941708221331966484585723722438709794411811359163268313938691329827951251267050560591642529907351637378159060836729528663195017591659-1)
```

This outputs the following:

```
[2, 897696227, 4079175531008894722165264101205435926648149833274524116828226243263002635029421089083973587454582813212933133186660564875794091457642235099125127]
```

Thus, we can see that the graphic in the paper right below the previously quoted text accurately displays the situation. `p-1` and `q-1` each factor into 2, a small prime, and a large prime. However, if we take our value for `p-1` and get it's prime factors by running the following command:

```
prime_factors(0xec4198b499d71ea60b224a4a9f0f04576fa8fd36485e05fd79a6ff1527be325a7a598341bbbedcd728b745525cc4b569f91a631ef74ee44f319e5f4d38bf3b9cb3d14b1a6e629553b831987695d0976a76a24860a23a7ebec42cbe41c625c8013e174ce1d19804e4b7111d8adab1a4690b5341c5897fcd33163077f07a4d0a17-1)
```

This produces a list of 67 different factors. Therefore, our values for the challenge cannot be broken using this method and we will need to use CM-HSO. Looking at the examples in the `test_CM_HSO` function in the test script, we see that a list of factors is provided, which is exactly what we have.

Luckily for us, David Wong has written most of the SageMath code for us in the `test_CM_HSO` function in the test script. If you look at our [solve.sage](./solve.sage) script, you'll notice that almost all of it is copied pasted from the `test_CM_HSO` function. On lines 1-12 of our script, we define the values we were given and that we figured out so far, then we calculate `p-1` and `q-1`, and finally we compute the prime factors of `p-1` and `q-1`. Then, we simply apply the algorithm discussed in the paper and presentation to go through the many small subgroups and reconstruct the private key. At the end we print the private key solution and we compute $c=g^x\bmod n$ (where $x$ is the private key that we found). This is exactly how the $c$ value given to us was computed so if the values are the same then we have found a valid private key. Running the script with `sage solve.sage` produces the following output:

```
attempting pollard rho in subgroup of order 32969
found it! 3138
attempting pollard rho in subgroup of order 33199
found it! 6803
attempting pollard rho in subgroup of order 33871
found it! 27378
attempting pollard rho in subgroup of order 34057
found it! 16222
attempting pollard rho in subgroup of order 34337
found it! 15166
attempting pollard rho in subgroup of order 34747
found it! 13210
attempting pollard rho in subgroup of order 35023
found it! 23279
attempting pollard rho in subgroup of order 35069
found it! 3463
attempting pollard rho in subgroup of order 35291
found it! 12728
attempting pollard rho in subgroup of order 36353
found it! 31678
attempting pollard rho in subgroup of order 36467
found it! 12587
attempting pollard rho in subgroup of order 36479
found it! 28933
attempting pollard rho in subgroup of order 36571
found it! 31442
attempting pollard rho in subgroup of order 36653
found it! 7804
attempting pollard rho in subgroup of order 36833
found it! 21713
attempting pollard rho in subgroup of order 37277
found it! 26965
attempting pollard rho in subgroup of order 38501
found it! 29378
attempting pollard rho in subgroup of order 38677
found it! 33985
attempting pollard rho in subgroup of order 39313
found it! 38773
attempting pollard rho in subgroup of order 39397
found it! 21112
attempting pollard rho in subgroup of order 39443
found it! 9720
attempting pollard rho in subgroup of order 39581
found it! 3449
attempting pollard rho in subgroup of order 41411
found it! 28647
attempting pollard rho in subgroup of order 41953
found it! 26225
attempting pollard rho in subgroup of order 42533
found it! 36616
attempting pollard rho in subgroup of order 43261
found it! 41105
attempting pollard rho in subgroup of order 43313
found it! 7467
attempting pollard rho in subgroup of order 43591
found it! 8617
attempting pollard rho in subgroup of order 43987
found it! 10465
attempting pollard rho in subgroup of order 44449
found it! 3691
attempting pollard rho in subgroup of order 44729
found it! 10888
attempting pollard rho in subgroup of order 44771
found it! 16800
attempting pollard rho in subgroup of order 46153
found it! 40391
attempting pollard rho in subgroup of order 46439
found it! 25479
attempting pollard rho in subgroup of order 47869
found it! 16057
attempting pollard rho in subgroup of order 47933
found it! 2027
attempting pollard rho in subgroup of order 48073
found it! 26661
attempting pollard rho in subgroup of order 48109
found it! 28805
attempting pollard rho in subgroup of order 48337
found it! 27143
attempting pollard rho in subgroup of order 48847
found it! 29834
attempting pollard rho in subgroup of order 50153
found it! 1554
attempting pollard rho in subgroup of order 50821
found it! 25844
attempting pollard rho in subgroup of order 51307
found it! 27336
attempting pollard rho in subgroup of order 53419
found it! 8474
attempting pollard rho in subgroup of order 53479
found it! 27939
attempting pollard rho in subgroup of order 53527
found it! 36332
attempting pollard rho in subgroup of order 53773
found it! 25027
attempting pollard rho in subgroup of order 55339
found it! 49784
attempting pollard rho in subgroup of order 55987
found it! 40592
attempting pollard rho in subgroup of order 56501
found it! 42478
attempting pollard rho in subgroup of order 57037
found it! 39119
attempting pollard rho in subgroup of order 58511
found it! 53382
attempting pollard rho in subgroup of order 58787
found it! 42045
attempting pollard rho in subgroup of order 59149
found it! 22613
attempting pollard rho in subgroup of order 59509
found it! 56896
attempting pollard rho in subgroup of order 59651
found it! 26731
attempting pollard rho in subgroup of order 60127
found it! 3969
attempting pollard rho in subgroup of order 60509
found it! 48595
attempting pollard rho in subgroup of order 60757
found it! 33035
attempting pollard rho in subgroup of order 60859
found it! 60326
attempting pollard rho in subgroup of order 61211
found it! 5171
attempting pollard rho in subgroup of order 61403
found it! 28853
attempting pollard rho in subgroup of order 61949
found it! 2563
attempting pollard rho in subgroup of order 62233
found it! 13222
attempting pollard rho in subgroup of order 63059
found it! 53457
attempting pollard rho in subgroup of order 65537
found it! 853
attempting pollard rho in subgroup of order 2
found it! 0
attempting pollard rho in subgroup of order 37463
found it! 32526
attempting pollard rho in subgroup of order 40841
found it! 37179
attempting pollard rho in subgroup of order 66301
found it! 56479
attempting pollard rho in subgroup of order 69761
found it! 15862
attempting pollard rho in subgroup of order 70271
found it! 21801
attempting pollard rho in subgroup of order 70709
found it! 57423
attempting pollard rho in subgroup of order 70793
found it! 57075
attempting pollard rho in subgroup of order 71011
found it! 27109
attempting pollard rho in subgroup of order 71119
found it! 24383
attempting pollard rho in subgroup of order 71837
found it! 41049
attempting pollard rho in subgroup of order 71999
found it! 26478
attempting pollard rho in subgroup of order 72577
found it! 22624
attempting pollard rho in subgroup of order 72613
found it! 56955
attempting pollard rho in subgroup of order 74017
found it! 43995
attempting pollard rho in subgroup of order 75079
found it! 64183
attempting pollard rho in subgroup of order 76481
found it! 74708
attempting pollard rho in subgroup of order 77471
found it! 61962
attempting pollard rho in subgroup of order 79181
found it! 13567
attempting pollard rho in subgroup of order 79687
found it! 31929
attempting pollard rho in subgroup of order 80737
found it! 28593
attempting pollard rho in subgroup of order 86239
found it! 6825
attempting pollard rho in subgroup of order 86257
found it! 21696
attempting pollard rho in subgroup of order 86453
found it! 10723
attempting pollard rho in subgroup of order 86627
found it! 85635
attempting pollard rho in subgroup of order 86923
found it! 15714
attempting pollard rho in subgroup of order 88721
found it! 65496
attempting pollard rho in subgroup of order 89917
found it! 69756
attempting pollard rho in subgroup of order 90499
found it! 66724
attempting pollard rho in subgroup of order 91957
found it! 80062
attempting pollard rho in subgroup of order 92143
found it! 40036
attempting pollard rho in subgroup of order 93479
found it! 1544
attempting pollard rho in subgroup of order 94201
found it! 82603
attempting pollard rho in subgroup of order 94723
found it! 62261
attempting pollard rho in subgroup of order 95911
found it! 32259
attempting pollard rho in subgroup of order 96097
found it! 35941
attempting pollard rho in subgroup of order 96731
found it! 5847
attempting pollard rho in subgroup of order 98963
found it! 61709
attempting pollard rho in subgroup of order 99923
found it! 26926
attempting pollard rho in subgroup of order 100279
found it! 2273
attempting pollard rho in subgroup of order 101267
found it! 62222
attempting pollard rho in subgroup of order 101429
found it! 30064
attempting pollard rho in subgroup of order 101573
found it! 39819
attempting pollard rho in subgroup of order 106031
found it! 4672
attempting pollard rho in subgroup of order 110069
found it! 65204
attempting pollard rho in subgroup of order 113749
found it! 20388
attempting pollard rho in subgroup of order 119027
found it! 59274
attempting pollard rho in subgroup of order 119869
found it! 59499
attempting pollard rho in subgroup of order 120193
found it! 49167
attempting pollard rho in subgroup of order 122599
found it! 23173
attempting pollard rho in subgroup of order 122819
found it! 6486
attempting pollard rho in subgroup of order 122827
found it! 85458
attempting pollard rho in subgroup of order 124277
found it! 46803
attempting pollard rho in subgroup of order 124739
found it! 106534
attempting pollard rho in subgroup of order 125863
found it! 72720
attempting pollard rho in subgroup of order 125921
found it! 112608
attempting pollard rho in subgroup of order 127703
found it! 37477
attempting pollard rho in subgroup of order 127763
found it! 87762
attempting pollard rho in subgroup of order 128747
found it! 29553
attempting pollard rho in subgroup of order 129169
found it! 116065
attempting pollard rho in subgroup of order 129527
found it! 90796
attempting pollard rho in subgroup of order 130639
found it! 74813
attempting pollard rho in subgroup of order 131009
found it! 51990
sol:  0x358f1f2c0cab934e90fb3b126673ea3e2d26aa7050dd3983cefbad0dc8cf913e6229cc407e6c83fed59150d42c7240cb04d85088837b41661db33f7a6495476d1b09fc48b2b374e321ab2e1100cbe1801456a9b32de8746b18b412d46dd491db4b5bba7c8b43e4cefaf760ce9c7f4c14411e7f31ee82bec752c28588b370a9205b26ea0907c46d25cb2b0c6ac821ab0f0e6bcb0ed35f871e7bec89579c84f17456099638dde8b6519072bda848ea9185a0fdef75dca9e86ac36485b8e16ea372f4c9163c9f8ddb2c1096cb0b66eef3567e688df55149c1b9f64d5a3e0a936da8dfb34f28acc8c5a123fb459eef0a9bb89fd7fcf4aacc143b79dddf7b030fe8fe
encrypted message:  0x51099773fd2aafd5f84dfe649acbb3558797f58bdc643ac6ee6f0a6fa30031767966316201c36be69241d9d05d0bd181ced13809f57b0c0594f6b29ac74bc7906dae70a2808799feddc71cf5b28401100e5e7e0324b9d8b56e540c725fa4ef87b9e8d0f901630da5f7f181f6d5b4cdc00d5f5c3457674abcb0d0c173f381b92bdfb143c595f024b98b9900410d502c87dfc1633796d640cb5f780fa4b6f0414fb51e34700d9096caf07b36f4dcd3bb5a2d126f60d3a802959d6fadf18f4970756f3099e14fa6386513fb8e6cdda80fdc1c32a10f6cdb197857caf1d7abf3812e3d9dcda106fa87bac382d3e6fc216c55da02a0c45a482550acb2f58bea2cfa03
```

The encrypted messages match so we have found the correct key. So, we have solved the discrete logarithm $c=g^x\bmod n$ for $x$. However, trying to decode this to ascii produces gibberish. This is because [discrete logarithms can have multiple solutions](https://crypto.stackexchange.com/a/87729). Essentially, it has something to do with group theory. The [video explains](https://youtu.be/90EYVy35gsY?t=1353) quite nicely that $\phi(n)=(p-1)(q-1)$ (where $\phi(n)$ is the size of the group) if $n=pq$. According to the paper several properties of `p` and `q` result "in a non-cyclic group with an upper-bound on possible subgroup orders of $lcm(s,t)=\frac{(p-1)(q-1)}{2}$." So, `gmpy2.lcm(p-1, q-1) == ((p-1)*(q-1))//2`. Therefore, [Carmichael totient's](https://en.wikipedia.org/wiki/Carmichael_function) is $\lambda=\frac{(p-1)(q-1)}{2}$.  The Carmichael function $\lambda(n)$ is the exponent/order of the [multiplicative group of integers modulo n](https://en.wikipedia.org/wiki/Multiplicative_group_of_integers_modulo_n). ([This page](https://en.wikipedia.org/wiki/Carmichael%27s_totient_function_conjecture) explains more about the Carmichael function. We don't use Euler's totient because the Carmichael function is more generalized.) Therefore, we can add or subtract $\frac{(p-1)(q-1)}{2}$ from the solution our SageMath script found (because we have a [cyclical group](https://en.wikipedia.org/wiki/Cyclic_group#Modular_multiplication)) and then compute $c=g^x\bmod n$ with our modified solution and still get the original $c$ value.

I'm still unsure about the math behind this. So, I messed around with the [find_other_solutions.py](./find_other_solutions.py) script for a while and eventually found that subtracting $\frac{(p-1)(q-1)}{4}$ from the computed key worked successfully and got the flag.

### Simpler Solution

This method is not explained since it should be somewhat understandable if you read through the above method.

First, use Pollard's p-1 algorithm to calculate `p` and `q`. You can use the implementation in [factor.py](./factor.py).

Second, run [simple_solve.sage](simple_solve.sage), which will print the hexadecimal key, `0x7069636f4354467b31636139333835387d`, and the flag.

Essentially, all of Mr. Wong's code (splitting the problem into multiple parts and then using the CRT) is unnecessary for this challenge and sagemath's `discrete_log` function is powerful enough.

### Flag

`picoCTF{1ca93858}`
