import time
import base64
import hashlib
import sys
import secrets


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


def encrypt(plaintext, inner_txt, key):
    midpoint = len(plaintext) // 2

    first_part = plaintext[:midpoint]
    second_part = plaintext[midpoint:]
    modified_plaintext = first_part + inner_txt + second_part
    block_size = 16
    plaintext = pad(modified_plaintext, block_size)
    key_hash = hashlib.sha256(key).digest()

    ciphertext = b''

    for i in range(0, len(plaintext), block_size):
        block = plaintext[i:i + block_size]
        cipher_block = xor_bytes(block, key_hash)
        ciphertext += cipher_block

    return ciphertext


def pad(data, block_size):
    padding_length = block_size - len(data) % block_size
    padding = bytes([padding_length] * padding_length)
    return data + padding


def xor_bytes(a, b):
    return bytes(x ^ y for x, y in zip(a, b))


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


if __name__ == "__main__":
    #  text = sys.argv[1]
    #  main(text)
    key = b'\xffo\xb6\x94g\xe1%\xfd`\x1f-\x1b\x8a\xab\xc5\xb5\xech\x82Q\xe4\xda1}+\x1c\xf1\x91\x83z\x12;'
    encrypted_blockchain = b'\xb8\x948/\xad\x03d\xd8\xa0WOp\xf4\x92\x07&\xe3\x940|\xffYg\x8a\xf5XH\x7f\xa7\xc4\x06!\xb6\x90b}\xa9\x0ee\x8b\xf1P\x1bq\xa5\x98\x07v\xb3\x949-\xfaZ2\xdf\xa0\x02K \xf4\xc5Rp\xad\xc61}\xff\x02g\x8b\xf6X\x13"\xf0\xc3\x04s\xb3\xc6c!\xfaY2\xdd\xa7QI$\xa5\xc0\x0fp\xe6\x94e/\xa9\x034\x8c\xa3V\x1b#\xa1\x94T \xe4\x93b \xaaXb\xd7\xf6U\x1fw\xa7\xc0Uq\xb8\xdb1(\xaa\x0b7\xdb\xf6V\x1a%\xfc\xc2\x03\'\xe2\x971|\xaf\x0fd\x8d\xf0W\x1d~\xf3\xc0\x03"\xe4\xc2qq\xfaTE\xbb\x87\x1bH*\xab\xc2\\\x1b\xb3\xa5Sp\xcfRT\x8d\x95Q[%\x9c\xfeo\x11\xea\xbb1j\xad\x02e\xa7\x9e\x11i<\xa9\xebm>\xc2\xbd^*\xaeX0\xd6\xa4\x06H;\xf2\x94\x03&\xb0\xce6{\xfd\x0cc\xdd\xa5\x03\x1eu\xf7\xc4\x05q\xe2\xc30}\xa9^b\xd8\xa0\x04K"\xe9\x91\x07q\xe5\x95e}\xfaZ4\xd7\xf4V\x19#\xfc\xc5\x03|\xe1\xc5e*\xff\x026\x89\xf7WO%\xf5\x91\x03r\xb4\x929(\xf8\r0\x8a\xa4\x01\x1c\'\xf7\x95\x02 \xe1\xc12}\xfa_7\x89\xf5U\x19r\xf7\x8c\x07t\xe6\xc7cz\xa9\x0b6\x8b\xa7\x06Oq\xfc\x91Q%\xe6\x927(\xfc^0\xd7\xa5Y\x1bt\xa7\xc3\x00r\xb2\x948)\xff\x037\xde\xf6\x03\x1et\xa1\x94\x05\'\xb0\xc01(\xfd\x0f6\x8e\xa2\x06\x18~\xf5\xc25F'
    print(encrypt(encrypted_blockchain, b'', key))
