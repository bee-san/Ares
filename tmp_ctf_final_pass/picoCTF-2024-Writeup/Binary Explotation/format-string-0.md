# Description

Can you use your knowledge of format strings to make the customers happy? <br>
Download the binary here. <br>
Download the source here. <br>
Connect with the challenge instance here: <br>
nc mimas.picoctf.net 55122

# Solution

Here is a better formatted version of this writeup on [picoCTF Solutions website](https://picoctfsolutions.com/picoctf-2024-format-string-0).

Use, `nc mimas.picoctf.net 55122`, to connect. Once connected it gives three options to choose from:

`Breakf@st_Burger, Gr%114d_Cheese, Bac0n_D3luxe`

This is a format string challenge, `Gr%114d_Cheese`, is the only thing that has a format string (%11) in it. Next, these are the choices:

`Pe%to_Portobello, $outhwest_Burger, Cla%sic_Che%s%steak`

`Cla%sic_Che%s%steak` has %s in it so by choosing that it gives the flag.

Helpful resources: [Specifier table](https://cplusplus.com/reference/cstdio/printf/) and [manual page](https://man7.org/linux/man-pages/man3/printf.3.html)

Flag: `picoCTF{7h3_cu570m3r_15_n3v3r_SEGFAULT_dc...}`
