main :: IO()
main = interact
        $ unlines
        . map (unwords . map show)
        . map (\[i,n] -> solve i n)
        . map (map (read::String->Integer) . words)
        . tail
        . lines
            where
                solve i n = [i,n*(n+1) `div` 2,n*n,n*(n+1)]
