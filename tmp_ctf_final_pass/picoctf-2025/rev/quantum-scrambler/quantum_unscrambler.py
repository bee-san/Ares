if __name__ == "__main__":
    with open("result", "r") as f:
        cypher = f.read()
    cypher = eval(cypher) 
    flag = []
    print(cypher[len(cypher) - 2])
    flag = []
    for i in range(len(cypher)-2):
        flag.append(((cypher[i][0])))
        flag.append(((cypher[i][-1])))
        print(flag)
    flag.append(((cypher[-2][0])))
    flag.append(((cypher[-1][0])))
    print(flag)
    print(''.join([chr(int(char[2:], 16)) for char in flag]))