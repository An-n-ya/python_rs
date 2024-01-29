import dis


for i in range(255):
    if dis.opname[i][0] != '<':
        print(hex(i), dis.opname[i])
print(hex(dis.HAVE_ARGUMENT))
print(dis._inline_cache_entries)

