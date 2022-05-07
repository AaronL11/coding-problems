solve :: Int -> [Char] -> Char -> [Char] -> [Char]
solve s sums p [] = sums ++ show (s+1) ++ [p]
solve s sums p [a] = if a == p
                        then sums ++ show (s+2) ++ [p]
                        else solve 0 (sums ++ show (s+1) ++ [p]) a []
solve s sums p [a,b] = if a==p
                        then solve (s+1) sums a [b]
                        else solve 0 (sums ++ show (s+1) ++ [p]) a [b]
solve s sums p (a:chars) = if p == a
                              then solve (s+1) sums a chars 
                              else solve 0 (sums ++ show (s+1)++[p]) a chars

main :: IO()
main = interact $
        unlines
        . map (\chars -> solve 0 "" (head chars) (tail chars))
        . words


