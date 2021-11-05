solve :: [Int] -> String
solve [r,e,c]
  | (e-c) > r = "advertise"
  | (e-c)==r = "does not matter"
  | otherwise = "do not advertise"

main :: IO()
main = interact
	$ unlines
	. map (solve . map read .words)
	. tail
	. lines

