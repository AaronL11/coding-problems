solve(g:k:gs) = if k-1==g then g:solve (k:gs) else [g,k]
main :: IO ()
main = interact
	$ unlines
	. map (show . length . solve . map (read::String->Int))
	. map (tail . words)
	. tail
	. lines

