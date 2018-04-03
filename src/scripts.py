# -*- coding: utf-8 -*-

for s in ["build_hero([{}], d[{}], d[{}], d[{}], d[{}], d[{}])".format(

  ", ".join(["d[{}]".format(60+h*7+t) for t in range(7)])  
        
,20+h,30+h,50+h,10+h,40+h) for h in range(10)]:
    print("{},".format(s))
    
