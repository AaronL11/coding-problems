solve :: Int -> [Int] -> Bool
solve n [] = True
solve n [_] = True
solve n list = foldl (\b i -> if b
			      && 1 <= i
			      && i <= n-1
			      then True
			      else False) True
			      $ map abs
			      $ zipWith (-) (tail list) (list)

main :: IO ()
main = interact
    $ unlines
    . map (
	  (\b -> if b then "Jolly" else "Not Jolly")
    	. (\(n:rest) -> solve n rest)
        . map read
        . words)
    . lines




