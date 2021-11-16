solve :: Int -> Int
solve n = 

main :: IO ()
main = interact
	$ unlines
	. \[k,y] -> [k,show . solve (read::String->Integer $ y)]
	. tail
	. lines
