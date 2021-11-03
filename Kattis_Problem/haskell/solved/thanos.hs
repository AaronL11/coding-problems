solve :: [Int] -> Int
solve [] = 0
solve [p,r,f] =
	if p <= f
    	    then 1 + solve [(p*r),r,f]
	    else 0

main :: IO()
main = interact
	$ unlines
	. map (show . solve . map (read::String->Int) . words)
	. tail
	. lines

