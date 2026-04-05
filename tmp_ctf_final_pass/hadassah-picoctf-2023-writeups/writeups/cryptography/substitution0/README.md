<h1>substitution0</h1>
<b>Written by: Idan Cohen</b>
<p>This is the write-up for the challenge "substitution0" from picoCTF - cryptography.</p>

<h2>The Challenge</h2>
<h3>Description</h3>
<p>A message has come in but it seems to be all scrambled. Luckily it seems to have the key at the beginning. 
Can you crack this substitution cipher? Download the message here.</p>
<img width="320" alt="substitution0_description" src="https://github.com/slashben/hadassah-picoctf-2023-writeups/assets/48062272/89e5dced-2515-42c1-b223-82d92610e119">
 
<h3>Hint</h3>
<p>Try a frequency attack. An online tool might help.</p>

<h3>Initial Look</h3>
<p>The message in the attached file was: </p>

<p><i>
OHNFUMWSVZLXEGCPTAJDYIRKQB 
<br>
Suauypcg Xuwaogf oacju, rvds o waoiu ogf jdoduxq ova, ogf hacywsd eu dsu huudxu
mace o wxojj noju vg rsvns vd roj ugnxcjuf. Vd roj o huoydvmyx jnoaohouyj, ogf, od
dsod dveu, yglgcrg dc godyaoxvjdj—cm ncyaju o wauod pavbu vg o jnvugdvmvn pcvgd
cm ivur. Dsuau ruau drc acygf hxonl jpcdj guoa cgu ukdauevdq cm dsu honl, ogf o
xcgw cgu guoa dsu cdsua. Dsu jnoxuj ruau uknuufvgwxq soaf ogf wxcjjq, rvds oxx dsu
oppuoaognu cm hyagvjsuf wcxf. Dsu ruvwsd cm dsu vgjund roj iuaq aueoalohxu, ogf,
dolvgw oxx dsvgwj vgdc ncgjvfuaodvcg, V ncyxf soafxq hxoeu Zypvdua mca svj cpvgvcg
aujpundvgw vd.
<br>
Dsu mxow vj: pvncNDM{5YH5717Y710G_3I0XY710G_03055505}
</i></p>


<h2>How to solve it</h2>
<p>As the name of the challenge suggests, this text is an enciphered text that was enciphered by substitution encryption. 
  In substitution cipher, every letter is substituted (hence the name) by another letter according to a given key.</p>
  
<p>In our message, the key was given in the first line: OHNFUMWSVZLXEGCPTAJDYIRKQB.
The meaning of this key is that the letter 'a' in the original plain text was encrypted to 'o';
  'b' was encrypted to 'h'; 'c' was encrypted to 'n' etc.</p>
  
<p>So in order to decipher the message and reveal the key, I decided to write a small python program the uses the decryption key and decipher the text. 
  In this program I move over the letters in the ciphered text, and replace every letter according to the decryption key.</p>
 
<p> The program is attached in the file "substitution0.py", and here is a screenshot of the program:</p>
<img width="601" alt="substitution0_code" src="https://github.com/slashben/hadassah-picoctf-2023-writeups/assets/48062272/06241c69-4c60-4ca9-9c7b-a5dab270a4c6">

<p>I run the program and reveal the text:</p>

<p><i>
HEREUPON LEGRAND AROSE, WITH A GRAVE AND STATELY AIR, AND BROUGHT ME THE BEETLE 
FROM A GLASS CASE IN WHICH IT WAS ENCLOSED. IT WAS A BEAUTIFUL SCARABAEUS, AND, AT
THAT TIME, UNKNOWN TO NATURALISTS—OF COURSE A GREAT PRIZE IN A SCIENTIFIC POINT
OF VIEW. THERE WERE TWO ROUND BLACK SPOTS NEAR ONE EXTREMITY OF THE BACK, AND A
LONG ONE NEAR THE OTHER. THE SCALES WERE EXCEEDINGLY HARD AND GLOSSY, WITH ALL THE
APPEARANCE OF BURNISHED GOLD. THE WEIGHT OF THE INSECT WAS VERY REMARKABLE, AND,
TAKING ALL THINGS INTO CONSIDERATION, I COULD HARDLY BLAME JUPITER FOR HIS OPINION
RESPECTING IT.
<br>
THE FLAG IS: PICOCTF{5UB5717U710N_3V0LU710N_03055505}
</i></p>
  
<p>It turned to be a citation from Edgar Allan Poe's story: " The Gold-Bug".</p>
<p>The picoCTF flag appears in the last line of the ciphered text: PICOCTF{5UB5717U710N_3V0LU710N_03055505}</p>
