s.s s:sstttss:l # PUSH c ON STACK
s.s s:sstttst:l # PUSH b ON STACK
s.s s:sstttts:l # PUSH a ON STACK
t.lss # a
t.lss # b
t.lss # c

# LINE FEED
s.s s:stst:l
tlss

s.s s:stttttt:l # PUSH 64 ON STACK
s.ls
s.ls
s.ls
s.ls
s.ls
t.lst # 2048

# LINE FEED
s.s s:stst:l
t.lss

s.s s:s:l # PUSH 1
s.s s:st:l # PUSH 2
s.lt       # SWAP TOP TWO
t.lst      # 1
t.lst      # 2

# LINE FEED
s.s s:stst:l
t.lss

s.s s:s:l
s.s s:st:l
s.ll
t.lst

# LINE FEED
s.s s:stst:l
t.lss

s.s s:s:l
s.s s:st:l
s.s s:ss:l
s.s s:stt:l
s.s s:sts:l
s.tl s:st l
t.lst # 5
t.lst # 2
t.lst # 1

# LINE FEED
s.s s:stst:l
t.lss

s.s s:s:l
s.s s:st:l
s.s s:ss:l
s.s s:stt:l
s.s s:sts:l
s.s s:sst:l
s.s s:sss:l
s.s s:sttt:l
s.ts s:sts:l
t.lst
t.lst
t.lst
t.lst
t.lst
t.lst
t.lst
t.lst
t.lst

# LINE FEED
s.s s:stst:l
t.lss

l.ll
