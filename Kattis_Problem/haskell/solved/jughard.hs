main :: IO ()
main = interact
	$ unlines
	. map (
		(\[a,b,d] -> if d `mod` (gcd a b) == 0
		  		then "Yes"
				else "No")
		. map read
		. words
       	)
	. tail
	. lines

