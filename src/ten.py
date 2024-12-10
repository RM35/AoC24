from pprint import pprint
from collections import deque

data = open("input").readlines()

mapp = []
for line in data:
    toadd = []
    for ch in line.strip():
        if ch == '.':
            toadd.append('.')
            continue
        toadd.append(int(ch))
    mapp.append(toadd)
    

pprint(mapp)

starts = set()
for y in range(len(mapp)):
    for x in range(len(mapp[y])):
        if mapp[y][x] == 0:
            starts.add((x, y))

pprint(starts)

total = 0
for xxx, yyy in starts: 
    total_section = 0
    # print(xxx, yyy)
    bfsq = deque()
    bfsq.append((xxx, yyy))

    already_visited = set()
    already_visited.add((xxx, yyy))

    counted = set()

    debug = False
    while bfsq:
        curx, cury = bfsq.popleft()
        if curx == 0 and cury == 6:
            debug = True

        curval = mapp[cury][curx]
        if debug:
            print(curx, cury, curval)
            print(bfsq)
        if curval == 9:
            if debug: print("Adding trailhead")
            total_section += 1
            counted.add((curx, cury))

        dirs = [(1, 0), (0, 1), (-1, 0), (0, -1)]
        for xx, yy in dirs:
            if (xx, yy) in already_visited:
                continue
            checkx = curx+xx
            checky = cury+yy
            try:
                if checkx < 0 or checky < 0: continue
                checkval = mapp[checky][checkx]
            except Exception: 
                continue
            if curval - checkval == -1:
                bfsq.append((checkx, checky))
    print(total_section)
    total += total_section

print(total)
