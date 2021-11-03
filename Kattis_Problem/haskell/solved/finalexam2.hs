main :: IO ()
main = interact
	$ show
	. length
	. (\ls -> filter (uncurry (==)) $ zip ls (tail ls))
	. tail
	. lines

