g = 3
n = 0xd63c7cb032ae4d3a43ecec4999cfa8f8b49aa9c14374e60f3beeb437233e44f988a73101f9b20ffb56454350b1c9032c136142220ded059876ccfde992551db46c27f122cacdd38c86acb844032f8600515aa6ccb7a1d1ac62d04b51b752476d2d6ee9f22d0f933bebdd833a71fd30510479fcc7ba0afb1d4b0a1622cdc2a48341010dffdcfc8d9af45959fb30b692dc2c9e181ac6bcd6a701326e3707fb19b7f9dfe1c522c68f9b0d229d384be1e1c58f72f8df60ca5172a341a7ee81428a064beedd6af7b89cc6079f2b6d3717f0d29330f0a70acca05bf67ab60c2e5cb0b86bfca2c9b8d50d79d24371432a1efb243f3c5f15b377ccc51f6e69bfbf5ecc61
y = 0x51099773fd2aafd5f84dfe649acbb3558797f58bdc643ac6ee6f0a6fa30031767966316201c36be69241d9d05d0bd181ced13809f57b0c0594f6b29ac74bc7906dae70a2808799feddc71cf5b28401100e5e7e0324b9d8b56e540c725fa4ef87b9e8d0f901630da5f7f181f6d5b4cdc00d5f5c3457674abcb0d0c173f381b92bdfb143c595f024b98b9900410d502c87dfc1633796d640cb5f780fa4b6f0414fb51e34700d9096caf07b36f4dcd3bb5a2d126f60d3a802959d6fadf18f4970756f3099e14fa6386513fb8e6cdda80fdc1c32a10f6cdb197857caf1d7abf3812e3d9dcda106fa87bac382d3e6fc216c55da02a0c45a482550acb2f58bea2cfa03

p = 0xec4198b499d71ea60b224a4a9f0f04576fa8fd36485e05fd79a6ff1527be325a7a598341bbbedcd728b745525cc4b569f91a631ef74ee44f319e5f4d38bf3b9cb3d14b1a6e629553b831987695d0976a76a24860a23a7ebec42cbe41c625c8013e174ce1d19804e4b7111d8adab1a4690b5341c5897fcd33163077f07a4d0a17
q = 0xe823cd272413ba5dbc8ade057120e2488345eea930e0b42f97d949c36e29218c2760059fef64d97da2a06144cb72e6451260d7e8f6d3cb78641131bdc2c8c09dc4f9395e0b1e9ac20d1266c9058b8c0e22ec7071236b1ab559188ed23de93213af1819453419f2108b453d3c9342e99a5a16e68acfe19b69af4b58b019a70047

p_order = p-1
q_order = q-1

p_factors = prime_factors(p_order)
q_factors = prime_factors(q_order)


# Pohlig-Hellman in (p-1)/2
yp = y % p
xp = 0
xp_mod = 1

for order in p_factors[1:]: # to remove the 2
    print("attempting pollard rho in subgroup of order", order)
    # reduce the problem
    new_problem = power_mod(yp, (p-1)//order, p)
    # find a generator of that group
    new_generator = power_mod(g, (p-1)//order, p)
    # Pollard Rho
    new_problem = GF(p)(new_problem)
    new_generator = GF(p)(new_generator)
    new_xp = discrete_log_rho(new_problem, new_generator, order)
    #
    print("found it!", new_xp)
    xp = CRT(xp, new_xp, xp_mod, order)
    xp_mod *= order

# Pohlig-Hellman in (q-1)
yq = y % q
xq = 0
xq_mod = 1

for order in q_factors: # we need the 2
    print("attempting pollard rho in subgroup of order", order)
    # reduce the problem
    new_problem = power_mod(yq, (q-1)//order, q)
    # find a generator of that group
    new_generator = power_mod(g, (q-1)//order, q)
    # Pollard Rho
    new_problem = GF(q)(new_problem)
    new_generator = GF(q)(new_generator)
    new_xq = discrete_log_rho(new_problem, new_generator, order)
    #
    print("found it!", new_xq)
    xq = CRT(xq, new_xq, xq_mod, order)
    xq_mod *= order

# CRT
sol = CRT(xp, xq, xp_mod, xq_mod)

print("sol: ", hex(sol))
print("encrypted message: ", hex(power_mod(g, sol, n)))
