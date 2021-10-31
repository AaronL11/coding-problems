import Data.List (sort, group)
main :: IO ()
main = interact
	$ show
	. foldl (\x y -> (y `div` 3)+x) 0
	. map length
	. group
	. sort
	. concat
	. words
