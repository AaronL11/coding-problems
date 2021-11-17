main :: IO ()
main = interact
	$ show
	. (\n -> (ceiling . logBase 2 $ n) + 1)
	. read

