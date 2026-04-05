# Torrent Analyze

## Challenge

SOS, someone is torrenting on our network. One of your colleagues has been using torrent to download some files on the company’s network. Can you identify the file(s) that were downloaded? The file name will be the flag, like `picoCTF{filename}`. [Captured traffic](https://artifacts.picoctf.net/c/206/torrent.pcap) ([Archive](https://web.archive.org/web/20220325004713/https://artifacts.picoctf.net/c/206/torrent.pcap)).

## Solution

Opening the file in [Wireshark](https://www.wireshark.org/) we see some TLS, TCP, DNS, and UDP connections. DNS requests are made to `torrent.ubuntu.com` and `ipv6.torrent.ubuntu.com`. We can visit those domains and find a [tracker index](https://ipv6.torrent.ubuntu.com/tracker_index) with `.torrent` files available for download.

Looking at the [description of a `.torrent` file on Wikipedia](https://en.wikipedia.org/wiki/Torrent_file) gives us a general overview of the file structure, but isn't overly helpful. [The article linked in the hints](https://www.techworm.net/2017/03/seeds-peers-leechers-torrents-language.html) explains that "A user who wants to upload a file first creates a small torrent descriptor file that they distribute by conventional means (web, email, etc.). They then make the file itself available through a BitTorrent node acting as a seed." This indicates that the `.torrent` file would contain the file name. If we download a random torrent file from the Ubuntu [tracker index](https://ipv6.torrent.ubuntu.com/tracker_index), we see that this is indeed the case. However, HTTPS is used to download this file so without the TLS key, which we don't have, we cannot read this file.

After looking through the UDP streams in Wireshark using the `Right click > Follow > UDP Stream` we notice that there are several streams that look like the following in ASCII:

```
d1:ad2:id20:...AK..w]}..hf...:..9:info_hash20:.F|.....A6{."0....X.e1:q9:get_peers1:t4:gp.#1:y1:qed1:rd2:id20:..0[.'..7.*}..YE.e .5:nodes208:.f.I........<..>.$^R/*..i6..>+..	..._P+.
6$M...>.0.a.	RG....3...s.....G.M8.|......m..mk.o...W.Q...^..U..........i.VG.<
>.,Z..%..8..1.....8$&..........%..R....D....?pn.h...x..*..3.........-....`........C..8..t5:token8:........e1:t4:gp.#1:y1:re
```

We also searched online for "torrent pcap" and found [this article](https://www.malware-traffic-analysis.net/2013/09/14/index.html) ([Archive](https://web.archive.org/web/20220325011426/https://www.malware-traffic-analysis.net/2013/09/14/index.html)). The article states that we can "easily filter traffic to find any torrent hashes of the files being downloaded or shared." It also says that "a Google search on the torrent hash will often tell you what the file is," which sounds exactly like what we want since we want to know the name of the file being downloaded as a torrent.

However, Wireshark will not interpret the traffic as bittorrent traffic for some reason, so we have to manually find this torrent hash.

Searching for "BT-DHT" (which was given in a hint) finds the [Mainline DHT Wikipedia article](https://en.wikipedia.org/wiki/Mainline_DHT), which states "Mainline DHT is the name given to the Kademlia-based distributed hash table (DHT) used by BitTorrent clients to find peers via the BitTorrent protocol." It also says "The SHA-1 hash of a torrent, the infohash, is synonymous with a Kademlia key, which is used for finding peers (values) in the overlay network. To find peers in a swarm, a node sends a get_peers query with the infohash as key (equivalent to a Kademlia FIND_VALUE) to the closest known nodes (with respect to the key distance)." So, if we can figure out what the "info hash" is we can Google the hash (according to [the previously reference article](https://www.malware-traffic-analysis.net/2013/09/14/index.html)) and get the file name.

We use the Wireshark search feature to search packet bytes, narrow & wide, for the string `info_hash`. For example, we find the hash `17d62de1495d4404f6fb385bdfd7ead5c897ea22`, which is not the file we want since it doesn't end in `.iso`. Luckily, at the end of the file we see the same hex characters after the `info_hash:` text:

```
0000   00 50 56 f5 e4 05 00 0c 29 2d 4b 5e 08 00 45 00   .PV.....)-K^..E.
0010   00 7d 1d cf 40 00 40 11 71 1f c0 a8 49 84 25 bb   .}..@.@.q...I.%.
0020   7b 9a c8 d5 c8 d5 00 69 ab fc 64 31 3a 61 64 32   {......i..d1:ad2
0030   3a 69 64 32 30 3a 17 c1 ec 41 4b 95 fc 77 5d 7d   :id20:...AK..w]}
0040   dd cb 68 66 93 b7 86 3a c1 aa 39 3a 69 6e 66 6f   ..hf...:..9:info
0050   5f 68 61 73 68 32 30 3a e2 46 7c bf 02 11 92 c2   _hash20:.F|.....
0060   41 36 7b 89 22 30 dc 1e 05 c0 58 0e 65 31 3a 71   A6{."0....X.e1:q
0070   39 3a 67 65 74 5f 70 65 65 72 73 31 3a 74 34 3a   9:get_peers1:t4:
0080   67 70 c7 23 31 3a 79 31 3a 71 65                  gp.#1:y1:qe
```

From this hex-ascii dump, we see that the info hash is `e2467cbf021192c241367b892230dc1e05c0580e`. Searching for this hash finds [this page](https://linuxtracker.org/index.php?page=torrent-details&id=e2467cbf021192c241367b892230dc1e05c0580e), which states that the name is `ubuntu-19.10-desktop-amd64.iso`. There are also references to that name from other pages in the Google search results. So we have found the flag.

Searching for "d1:ad2" online (the start of the packets we kept seeing) reveals that is what a BitTorrent payload begins with. We also find the [DHT Protocol specification](http://bittorrent.org/beps/bep_0005.html), which explains how the `get_peers` function that we observed works (notice the `get_peers` in the ascii dumps).

### Flag

`picoCTF{ubuntu-19.10-desktop-amd64.iso}`
