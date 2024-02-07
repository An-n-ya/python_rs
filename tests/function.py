def foo(a, b = 1):
    print(a, b)
    return a

foo(a = 1, b = 2)
foo(b = 1, a = 2)
foo(2)
