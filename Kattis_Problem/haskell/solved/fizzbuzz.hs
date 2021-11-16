main :: IO()
main = interact
	$ unlines
	. (\[x,y,n] -> map (
			\i -> case () of
    				    _ | mod i x == 0
				     && mod i y == 0 -> "FizzBuzz"
				      | mod i x == 0 -> "Fizz"
  				      | mod i y == 0 -> "Buzz"
				      | otherwise    -> show i
		      	)
		$ [1..n])
	. map read
	. words

