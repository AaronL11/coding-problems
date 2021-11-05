main :: IO ()
main = interact
	$ (\i -> if 0<= i && i <= 100 then show i else "impossible")
	. (\[n,k,d,s] -> (n*d-k*s)/(n-k))
	. map read
	. words

