long long check(char* input)
{
    if (strlen(input) != 0x1b)
        return 1;
    
    long long enc_flag;
    __builtin_memcpy(&enc_flag, "\xe1\xa7\x1e\xf8\x75\x23\x7b\x61\xb9\x9d\xfc\x5a\x5b\xdf\x69\xd2\xfe\x1b\xed\xf4\xed\x67\xf4", 0x17);
    long var_1c_1 = 0;
    long var_20_1 = 0;
    long var_2c_1 = 0;
    
    for (long i = 0; i <= 22; i += 1)
    {
        for (long j = 0; j <= 7; j += 1)
        {
            if (!var_20_1)
                var_20_1 += 1;
            
            long rax_17;
            rax_17 = (input[var_1c_1] & 1 << (7 - var_20_1)) > 0;
            
            if (rax_17 != (*(&enc_flag + i) & 1 << (7 - j)) > 0)
                return 1;
            
            var_20_1 += 1;
            
            if (var_20_1 == 8)
            {
                var_20_1 = 0;
                var_1c_1 += 1;
            }
            
            if (var_1c_1 == strlen(input))
                return 0;
        }
    }
    
    return 0;
}