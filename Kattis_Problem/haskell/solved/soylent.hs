main :: IO ()
main = interact
	$ unlines
	. map (show
		. (\n -> div n 400 + if mod n 400 == 0
			  		then 0
    					else 1)
		. read)
	. tail
	. lines

