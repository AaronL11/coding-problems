main :: IO ()
main = interact
	$ unlines
	. map (\s -> "Thank you, " ++ s ++ ", and farewell!")
	. lines

