main :: IO ()
main = interact
	$ show
	. fst
	. foldl (\(c,t) n -> if n > t then (c+1,n) else (c,n))
		((0,0))
	. map read
	. tail
	. words
