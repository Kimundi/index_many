l = []
for n in range(0, 20):
    c1 = 0
    c2 = 0
    for (i, idx) in enumerate(range(0, n)):
        #print(f"n={n}, check {i} against len")
        c1 += 1
        for idx2 in range(0, i):
            #print(f"n={n}, check idx={idx} against idx2={idx2}")
            c2 += 1
    print(f"n={n}, c1={c1}, c2={c2}, c={c1+c2}")
    l.append(c1+c2)
print(l)
