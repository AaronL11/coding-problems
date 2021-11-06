solve :: String -> String -> String
solve s c
  | s == "A#" = "Bb" ++ " " ++ c
  | s == "Bb" = "A#" ++ " " ++ c
  | s == "C#" = "Db" ++ " " ++ c
  | s == "Db" = "C#" ++ " " ++ c
  | s == "D#" = "Eb" ++ " " ++ c
  | s == "Eb" = "D#" ++ " " ++ c
  | s == "F#" = "Gb" ++ " " ++ c
  | s == "Gb" = "F#" ++ " " ++ c
  | s == "Ab" = "G#" ++ " " ++ c
  | s == "G#" = "Ab" ++ " " ++ c
  | otherwise = "UNIQUE"


main :: IO ()
main = interact
	$ unlines
	. map (\(n,s) -> "Case " ++ show n ++ ": " ++ s)
	. map (\(n,s) -> (n, solve (take 2 s) (drop 3 s)))
	. zip [1..5]
	. lines
