solve :: [Double] -> String
solve [x,y]
  | [x,y] == [0,1] 	= "ALL GOOD"
  | y == 1		= "IMPOSSIBLE"
  | otherwise 		= show (x/(1-y))

main :: IO()
main = interact
	$ solve
	. map read
	. words

