main :: IO ()
main = interact
	$ (\[a,b] -> if length a >= length b then "go" else "no")
	. lines

