main :: IO ()
main = interact
        $ show
	. (*100)
	. (^2)
	. (\[r,c] -> (r-c)/r)
	. map read
	. words

