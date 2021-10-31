main :: IO ()
main = interact
	$ unlines
	. map (show . length)
	. tail
	. words

