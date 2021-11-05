solve :: [Int] -> String
solve [a,b,c]
  | a+b==c = "Possible"
  | a-b==c = "Possible"
  | b-a==c = "Possible"
  | a*b==c = "Possible"
  | a==c*b = "Possible"
  | b==c*a = "Possible"
  | otherwise = "Impossible"

main :: IO()
main = interact
	$ unlines
	. map (solve . map read .words)
	. tail
	. lines

