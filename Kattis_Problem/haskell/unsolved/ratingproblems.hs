main :: IO()
main = interact
	$ unwords
	. map show
	. (\(k:n:ks) -> map (/k) [sum ks + (k-n)*(3),sum ks + (k-n)*(-3)])
	. map read
	. words

