solve :: String -> String
solve s
 | s=="OCT 31" = "yup"
 | s=="DEC 25" = "yup"
 | otherwise = "nope"


main :: IO ()
main = interact
	$ unlines
	. map solve
	. lines

