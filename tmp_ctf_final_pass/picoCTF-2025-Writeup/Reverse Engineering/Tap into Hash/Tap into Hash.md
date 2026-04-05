# Tap into Hash #
 
## Overview ##

200 points

Category: [Reverse Engineering](../)

Tags: `#reverseengineering #decrypt #xor #blockchain`

## Description ##

Can you make sense of this source code file and write a function that will decode the given encrypted file content? 
Find the encrypted file here. It might be good to analyze source file to get the flag.

## Approach ##

Analysing the provided `block_chain.py` source file ...........


?

## Solution ##

?


------------------------------------------------------------------------------



Output from decryption:

    $ python3 block_chain_dec.py enc_flag 
    b'\x8b\x9a\x00G\xfe\xb3\xf3\x93\xdb\xa8yT\xfe\x15\x87a\xf4\xdf\x00\x8d\xee\xab\xd9\t^|\x04(%\x81\x9e\xf8'
    Encrypted Blockchain: b'Z5Wo\xe9\xbd\xf4\xed<\xeb=\xcb%\xc4\xf0>S2\x0bl\xe9\xe9\xf0\xe8>\xe8n\x91q\xca\xad=\x01fRo\xbe\xba\xa2\xb5f\xb88\x90t\xc4\xaco\x041\x01n\xb3\xbd\xa1\xb4l\xee9\xc9u\x9e\xf1:O0\x035\xed\xeb\xf6\xean\xbei\xcc.\x9f\xfe8\x008\x04l\xef\xee\xf4\xe9g\xbf8\x9bt\x9f\xaanZ6\x0b8\xef\xbb\xa6\xb8>\xe9n\xcd$\xc5\xf08\x00eQ>\xef\xed\xf4\xeaf\xbdm\x9e \xcb\xfe9\x07-\x03=\xba\xb9\xa2\xbcl\xb4:\x9c&\x9e\xab8U4\x05=\xe8\xec\xa4\xeao\xefk\x9c#\x9a\xacn\x00bCd\xe8\xb7\xd6\xd8\x19\xf69\xc4x\x9f\xa2UQSae\xdd\xb1\xc7\xee\x0b\xbc*\xcbO\xa3\x91_\x08M\x03\x7f\xbf\xe1\xf6\xc4\x00\xfc\x18\xd2z\xb6\x93p Kl;\xbb\xee\xa1\xbb9\xef9\xd5t\x99\xf9;Ve\x04i\xb3\xe9\xf3\xbd;\xb8m\x91s\xcf\xff:\x019\x04=\xbd\xb9\xa4\xeff\xb5>\xcd:\xcc\xf9<Ta\x01?\xbd\xe9\xf0\xed=\xefi\x9fr\x98\xf8=[a\x00:\xbc\xba\xf3\xe9k\xeci\x90"\x98\xfbnZaPk\xed\xe9\xa6\xbf=\xe8>\x99.\xc5\xac?Wa\x02<\xbd\xb9\xa0\xeaj\xecn\x9b$\xd1\xf9:\x04dR:\xb9\xea\xa3\xedi\xebh\x9d\'\xcc\xfah\x008U9\xe8\xe8\xf6\xba>\xefk\x98#\xce\xfb;\x038\n?\xea\xb9\xf7\xbfg\xbbk\x90q\x9e\xacn[f\x06l\xef\xbb\xa7\xbaf\xbd9\x9a"\xcf\xcb\x08'

    DEBUG: xor_bytes: len(a)=16, len(b)=32
    DEBUG: xor_bytes: a=b'Z5Wo\xe9\xbd\xf4\xed<\xeb=\xcb%\xc4\xf0>', b=b'b\x003\r\x8b\xd8\x95\x8c_\x8d[\xa8\x17\xfc\xc9\n\xe2\xe1)\x1b3\xd5\x95\xe5"@\x9f\xf7\xd0\x81\xc9\xff'
    DEBUG: xor_bytes: xor_ab=b'85dbbeaacffc2894'
    DEBUG: xor_bytes: len(a)=16, len(b)=32
    DEBUG: xor_bytes: a=b'S2\x0bl\xe9\xe9\xf0\xe8>\xe8n\x91q\xca\xad=', b=b'b\x003\r\x8b\xd8\x95\x8c_\x8d[\xa8\x17\xfc\xc9\n\xe2\xe1)\x1b3\xd5\x95\xe5"@\x9f\xf7\xd0\x81\xc9\xff'
    DEBUG: xor_bytes: xor_ab=b'128ab1edae59f6d7'
    DEBUG: xor_bytes: len(a)=16, len(b)=32
    DEBUG: xor_bytes: a=b'\x01fRo\xbe\xba\xa2\xb5f\xb88\x90t\xc4\xaco', b=b'b\x003\r\x8b\xd8\x95\x8c_\x8d[\xa8\x17\xfc\xc9\n\xe2\xe1)\x1b3\xd5\x95\xe5"@\x9f\xf7\xd0\x81\xc9\xff'
    DEBUG: xor_bytes: xor_ab=b'cfab5b7995c8c8ee'
    DEBUG: xor_bytes: len(a)=16, len(b)=32
    DEBUG: xor_bytes: a=b'\x041\x01n\xb3\xbd\xa1\xb4l\xee9\xc9u\x9e\xf1:', b=b'b\x003\r\x8b\xd8\x95\x8c_\x8d[\xa8\x17\xfc\xc9\n\xe2\xe1)\x1b3\xd5\x95\xe5"@\x9f\xf7\xd0\x81\xc9\xff'
    DEBUG: xor_bytes: xor_ab=b'f12c8e483cbabb80'
    DEBUG: xor_bytes: len(a)=16, len(b)=32
    DEBUG: xor_bytes: a=b'O0\x035\xed\xeb\xf6\xean\xbei\xcc.\x9f\xfe8', b=b'b\x003\r\x8b\xd8\x95\x8c_\x8d[\xa8\x17\xfc\xc9\n\xe2\xe1)\x1b3\xd5\x95\xe5"@\x9f\xf7\xd0\x81\xc9\xff'
    DEBUG: xor_bytes: xor_ab=b'-008f3cf132d9c72'
    DEBUG: xor_bytes: len(a)=16, len(b)=32
    DEBUG: xor_bytes: a=b'\x008\x04l\xef\xee\xf4\xe9g\xbf8\x9bt\x9f\xaan', b=b'b\x003\r\x8b\xd8\x95\x8c_\x8d[\xa8\x17\xfc\xc9\n\xe2\xe1)\x1b3\xd5\x95\xe5"@\x9f\xf7\xd0\x81\xc9\xff'
    DEBUG: xor_bytes: xor_ab=b'b87ad6ae82c3cccd'
    DEBUG: xor_bytes: len(a)=16, len(b)=32
    DEBUG: xor_bytes: a=b'Z6\x0b8\xef\xbb\xa6\xb8>\xe9n\xcd$\xc5\xf08', b=b'b\x003\r\x8b\xd8\x95\x8c_\x8d[\xa8\x17\xfc\xc9\n\xe2\xe1)\x1b3\xd5\x95\xe5"@\x9f\xf7\xd0\x81\xc9\xff'
    DEBUG: xor_bytes: xor_ab=b'8685dc34ad5e3992'
    DEBUG: xor_bytes: len(a)=16, len(b)=32
    DEBUG: xor_bytes: a=b'\x00eQ>\xef\xed\xf4\xeaf\xbdm\x9e \xcb\xfe9', b=b'b\x003\r\x8b\xd8\x95\x8c_\x8d[\xa8\x17\xfc\xc9\n\xe2\xe1)\x1b3\xd5\x95\xe5"@\x9f\xf7\xd0\x81\xc9\xff'
    DEBUG: xor_bytes: xor_ab=b'beb3d5af90667773'
    DEBUG: xor_bytes: len(a)=16, len(b)=32
    DEBUG: xor_bytes: a=b'\x07-\x03=\xba\xb9\xa2\xbcl\xb4:\x9c&\x9e\xab8', b=b'b\x003\r\x8b\xd8\x95\x8c_\x8d[\xa8\x17\xfc\xc9\n\xe2\xe1)\x1b3\xd5\x95\xe5"@\x9f\xf7\xd0\x81\xc9\xff'
    DEBUG: xor_bytes: xor_ab=b'e-001a7039a41bb2'
    DEBUG: xor_bytes: len(a)=16, len(b)=32
    DEBUG: xor_bytes: a=b'U4\x05=\xe8\xec\xa4\xeao\xefk\x9c#\x9a\xacn', b=b'b\x003\r\x8b\xd8\x95\x8c_\x8d[\xa8\x17\xfc\xc9\n\xe2\xe1)\x1b3\xd5\x95\xe5"@\x9f\xf7\xd0\x81\xc9\xff'
    DEBUG: xor_bytes: xor_ab=b'7460c41f0b044fed'
    DEBUG: xor_bytes: len(a)=16, len(b)=32
    DEBUG: xor_bytes: a=b'\x00bCd\xe8\xb7\xd6\xd8\x19\xf69\xc4x\x9f\xa2U', b=b'b\x003\r\x8b\xd8\x95\x8c_\x8d[\xa8\x17\xfc\xc9\n\xe2\xe1)\x1b3\xd5\x95\xe5"@\x9f\xf7\xd0\x81\xc9\xff'
    DEBUG: xor_bytes: xor_ab=b'bbpicoCTF{block_'
    DEBUG: xor_bytes: len(a)=16, len(b)=32
    DEBUG: xor_bytes: a=b'QSae\xdd\xb1\xc7\xee\x0b\xbc*\xcbO\xa3\x91_', b=b'b\x003\r\x8b\xd8\x95\x8c_\x8d[\xa8\x17\xfc\xc9\n\xe2\xe1)\x1b3\xd5\x95\xe5"@\x9f\xf7\xd0\x81\xc9\xff'
    DEBUG: xor_bytes: xor_ab=b'3SRhViRbT1qcX_XU'
    DEBUG: xor_bytes: len(a)=16, len(b)=32
    DEBUG: xor_bytes: a=b'\x08M\x03\x7f\xbf\xe1\xf6\xc4\x00\xfc\x18\xd2z\xb6\x93p', b=b'b\x003\r\x8b\xd8\x95\x8c_\x8d[\xa8\x17\xfc\xc9\n\xe2\xe1)\x1b3\xd5\x95\xe5"@\x9f\xf7\xd0\x81\xc9\xff'
    DEBUG: xor_bytes: xor_ab=b'jM0r49cH_qCzmJZz'
    DEBUG: xor_bytes: len(a)=16, len(b)=32
    DEBUG: xor_bytes: a=b' Kl;\xbb\xee\xa1\xbb9\xef9\xd5t\x99\xf9;', b=b'b\x003\r\x8b\xd8\x95\x8c_\x8d[\xa8\x17\xfc\xc9\n\xe2\xe1)\x1b3\xd5\x95\xe5"@\x9f\xf7\xd0\x81\xc9\xff'
    DEBUG: xor_bytes: xor_ab=b'BK_60647fbb}ce01'
    DEBUG: xor_bytes: len(a)=16, len(b)=32
    DEBUG: xor_bytes: a=b'Ve\x04i\xb3\xe9\xf3\xbd;\xb8m\x91s\xcf\xff:', b=b'b\x003\r\x8b\xd8\x95\x8c_\x8d[\xa8\x17\xfc\xc9\n\xe2\xe1)\x1b3\xd5\x95\xe5"@\x9f\xf7\xd0\x81\xc9\xff'
    DEBUG: xor_bytes: xor_ab=b'4e7d81f1d569d360'
    DEBUG: xor_bytes: len(a)=16, len(b)=32
    DEBUG: xor_bytes: a=b'\x019\x04=\xbd\xb9\xa4\xeff\xb5>\xcd:\xcc\xf9<', b=b'b\x003\r\x8b\xd8\x95\x8c_\x8d[\xa8\x17\xfc\xc9\n\xe2\xe1)\x1b3\xd5\x95\xe5"@\x9f\xf7\xd0\x81\xc9\xff'
    DEBUG: xor_bytes: xor_ab=b'c9706a1c98ee-006'
    DEBUG: xor_bytes: len(a)=16, len(b)=32
    DEBUG: xor_bytes: a=b'Ta\x01?\xbd\xe9\xf0\xed=\xefi\x9fr\x98\xf8=', b=b'b\x003\r\x8b\xd8\x95\x8c_\x8d[\xa8\x17\xfc\xc9\n\xe2\xe1)\x1b3\xd5\x95\xe5"@\x9f\xf7\xd0\x81\xc9\xff'
    DEBUG: xor_bytes: xor_ab=b'6a2261eabb27ed17'
    DEBUG: xor_bytes: len(a)=16, len(b)=32
    DEBUG: xor_bytes: a=b'[a\x00:\xbc\xba\xf3\xe9k\xeci\x90"\x98\xfbn', b=b'b\x003\r\x8b\xd8\x95\x8c_\x8d[\xa8\x17\xfc\xc9\n\xe2\xe1)\x1b3\xd5\x95\xe5"@\x9f\xf7\xd0\x81\xc9\xff'
    DEBUG: xor_bytes: xor_ab=b'9a377bfe4a285d2d'
    DEBUG: xor_bytes: len(a)=16, len(b)=32
    DEBUG: xor_bytes: a=b'ZaPk\xed\xe9\xa6\xbf=\xe8>\x99.\xc5\xac?', b=b'b\x003\r\x8b\xd8\x95\x8c_\x8d[\xa8\x17\xfc\xc9\n\xe2\xe1)\x1b3\xd5\x95\xe5"@\x9f\xf7\xd0\x81\xc9\xff'
    DEBUG: xor_bytes: xor_ab=b'8acff133bee199e5'
    DEBUG: xor_bytes: len(a)=16, len(b)=32
    DEBUG: xor_bytes: a=b'Wa\x02<\xbd\xb9\xa0\xeaj\xecn\x9b$\xd1\xf9:', b=b'b\x003\r\x8b\xd8\x95\x8c_\x8d[\xa8\x17\xfc\xc9\n\xe2\xe1)\x1b3\xd5\x95\xe5"@\x9f\xf7\xd0\x81\xc9\xff'
    DEBUG: xor_bytes: xor_ab=b'5a116a5f5a533-00'
    DEBUG: xor_bytes: len(a)=16, len(b)=32
    DEBUG: xor_bytes: a=b"\x04dR:\xb9\xea\xa3\xedi\xebh\x9d'\xcc\xfah", b=b'b\x003\r\x8b\xd8\x95\x8c_\x8d[\xa8\x17\xfc\xc9\n\xe2\xe1)\x1b3\xd5\x95\xe5"@\x9f\xf7\xd0\x81\xc9\xff'
    DEBUG: xor_bytes: xor_ab=b'fda7226a6f35003b'
    DEBUG: xor_bytes: len(a)=16, len(b)=32
    DEBUG: xor_bytes: a=b'\x008U9\xe8\xe8\xf6\xba>\xefk\x98#\xce\xfb;', b=b'b\x003\r\x8b\xd8\x95\x8c_\x8d[\xa8\x17\xfc\xc9\n\xe2\xe1)\x1b3\xd5\x95\xe5"@\x9f\xf7\xd0\x81\xc9\xff'
    DEBUG: xor_bytes: xor_ab=b'b8f4c0c6ab004221'
    DEBUG: xor_bytes: len(a)=16, len(b)=32
    DEBUG: xor_bytes: a=b'\x038\n?\xea\xb9\xf7\xbfg\xbbk\x90q\x9e\xacn', b=b'b\x003\r\x8b\xd8\x95\x8c_\x8d[\xa8\x17\xfc\xc9\n\xe2\xe1)\x1b3\xd5\x95\xe5"@\x9f\xf7\xd0\x81\xc9\xff'
    DEBUG: xor_bytes: xor_ab=b'a892aab38608fbed'
    DEBUG: xor_bytes: len(a)=16, len(b)=32
    DEBUG: xor_bytes: a=b'[f\x06l\xef\xbb\xa7\xbaf\xbd9\x9a"\xcf\xcb\x08', b=b'b\x003\r\x8b\xd8\x95\x8c_\x8d[\xa8\x17\xfc\xc9\n\xe2\xe1)\x1b3\xd5\x95\xe5"@\x9f\xf7\xd0\x81\xc9\xff'
    DEBUG: xor_bytes: xor_ab=b'9f5adc2690b253\x02\x02'
    DEBUG:decrypt: num_pad=2
    Decrypted Token: b'picoCTF{block_3SRhViRbT1qcX_XUjM0r49cH_qCzmJZzBK_60647fbb}'

Decrypt version of script:

    $ cat block_chain_dec.py 
    import time
    import base64
    import hashlib
    import sys
    import secrets

    import binascii
    import codecs


    class Block:
        def __init__(self, index, previous_hash, timestamp, encoded_transactions, nonce):
            self.index = index
            self.previous_hash = previous_hash
            self.timestamp = timestamp
            self.encoded_transactions = encoded_transactions
            self.nonce = nonce

        def calculate_hash(self):
            block_string = f"{self.index}{self.previous_hash}{self.timestamp}{self.encoded_transactions}{self.nonce}"
            return hashlib.sha256(block_string.encode()).hexdigest()


    def proof_of_work(previous_block, encoded_transactions):
        index = previous_block.index + 1
        timestamp = int(time.time())
        nonce = 0

        block = Block(index, previous_block.calculate_hash(),
                      timestamp, encoded_transactions, nonce)

        while not is_valid_proof(block):
            nonce += 1
            block.nonce = nonce

        return block


    def is_valid_proof(block):
        guess_hash = block.calculate_hash()
        return guess_hash[:2] == "00"


    def decode_transactions(encoded_transactions):
        return base64.b64decode(encoded_transactions).decode('utf-8')


    def get_all_blocks(blockchain):
        return blockchain


    def blockchain_to_string(blockchain):
        block_strings = [f"{block.calculate_hash()}" for block in blockchain]
        return '-'.join(block_strings)


              # blockchain_string, token, key
    def encrypt(plaintext, inner_txt, key):
        midpoint = len(plaintext) // 2

        first_part = plaintext[:midpoint]
        second_part = plaintext[midpoint:]
        modified_plaintext = first_part + inner_txt + second_part
        print('DEBUG:encrypt: len(first_part)=' + str(len(first_part)) + ', len(inner_txt)=' + str(len(inner_txt)) + ', len(second_part)=' + str(len(second_part)))
        block_size = 16
        print('DEBUG:encrypt: plaintext(before pad)=' + str(plaintext))
        plaintext = pad(modified_plaintext, block_size)
        print('DEBUG:encrypt: plaintext(after pad)=' + str(plaintext))    
        key_hash = hashlib.sha256(key).digest()
        print('DEBUG:encrypt: key=' + str(key) + ', key_hash=' + str(key_hash))

        ciphertext = b''

        for i in range(0, len(plaintext), block_size):
            print('DEBUG:encrypt: i=' + str(i))
            block = plaintext[i:i + block_size]
            cipher_block = xor_bytes(block, key_hash)
            # print(str(len(cipher_block)))
            ciphertext += cipher_block

        return ciphertext

    def decrypt(ciphertext, key):
        block_size = 16
        inner_text = b''
        # calculate key_hash
        key_hash = hashlib.sha256(key).digest()    
        # loop over cipher blocks (block_size)
        for i in range(0, len(ciphertext), block_size):
            block = ciphertext[i:i + block_size]
            plaintext_block = xor_bytes(block, key_hash)
            inner_text += plaintext_block
        # remove trailing padding
        num_pad = ord(inner_text[-1:])
        print('DEBUG:decrypt: num_pad=' +  str(num_pad))
        inner_text = inner_text[:-num_pad]
        # remove the blockchain_string pre and post ambles
        inner_text = inner_text[162:]
        inner_text = inner_text[:-162]
        return inner_text


    def pad(data, block_size):
        padding_length = block_size - len(data) % block_size
        padding = bytes([padding_length] * padding_length)
        return data.encode() + padding

                # block, key_hash
    def xor_bytes(a, b):
        print('DEBUG: xor_bytes: len(a)=' + str(len(a)) + ', len(b)=' + str(len(b)))
        print('DEBUG: xor_bytes: a=' + str(a) + ', b=' + str(b))
        xor_ab = bytes(x ^ y for x, y in zip(a, b))
        print('DEBUG: xor_bytes: xor_ab=' + str(xor_ab))
        return xor_ab


    def generate_random_string(length):
        return secrets.token_hex(length // 2)


    random_string = generate_random_string(64)


    def main(token):
        key = bytes.fromhex(random_string)

        print("Key:", key)

        genesis_block = Block(0, "0", int(time.time()), "EncodedGenesisBlock", 0)
        blockchain = [genesis_block]

        for i in range(1, 5):
            encoded_transactions = base64.b64encode(
                f"Transaction_{i}".encode()).decode('utf-8')
            new_block = proof_of_work(blockchain[-1], encoded_transactions)
            blockchain.append(new_block)

        all_blocks = get_all_blocks(blockchain)

        blockchain_string = blockchain_to_string(all_blocks)
        encrypted_blockchain = encrypt(blockchain_string, token, key)

        print("Encrypted Blockchain:", encrypted_blockchain)

        # decrypted_token = decrypt(encrypted_blockchain, key)
        # print("Decrypted Token:", decrypted_token)

    def main2(enc_file):
        # open enc_file
        with open(enc_file, 'r') as file:
            # extract "Key: <>"
            key_line = file.readline()
            #print(key_line[7:-2])
            #print(codecs.escape_decode(key_line[7:-2])[0])
            key = codecs.escape_decode(key_line[7:-2])[0]
            #print(hex(key_line[6]))
            #print((key_line[7:-2]).decode('hex'))
            #key = (key_line[5:]).encode()
            #key = binascii.unhexlify(bytes(key_line[7:-1], encoding='utf-8'))
            # extract "Encrypted Blockchain: <>"
            enc_blockchain_line = file.readline()
            encrypted_blockchain = codecs.escape_decode(enc_blockchain_line[24:-2])[0]

            print(key)``
            print(enc_blockchain_line)

            decrypted_token = decrypt(encrypted_blockchain, key)
            print("Decrypted Token:", decrypted_token)

    if __name__ == "__main__":
        text = sys.argv[1]
        #main(text)
        main2(text)

Really just loading key and cipher text from file, xor'ing to reverse, chopping back up into blocks, removing padding and pre-post ambles (refer comments in decrypt script).

------------------------------------------------------------------------------

