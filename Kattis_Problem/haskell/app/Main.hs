
solve' :: String -> String -> String -> String
solve' = undefined

solve :: [[String]] -> [String]
solve [] = []
solve ([s,n,l]:rest) = solve' s n l : solve rest
solve [_] = undefined

main :: IO()
main = interact
	$ unlines
	. solve
	. tail
	. lines
