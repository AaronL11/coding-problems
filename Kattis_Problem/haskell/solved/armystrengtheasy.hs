winner :: [Int] -> [Int] -> String
winner b c = if mx b >= mx c then "Godzilla" else "MechaGodzilla"
	where
		mx = foldl (max) 0

solve :: [[Int]] -> [String]
solve [] = []
solve (a:b:c:rest) = winner b c : solve rest

main :: IO()
main = interact
	$ unlines
	. solve
	. map (map read . words)
	. filter (/="")
	. tail
	. lines

