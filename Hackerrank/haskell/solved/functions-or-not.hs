solve :: [Int] -> [String] -> [Bool]
solve set (x:xs) = (solve' set xs') : (solve set $ drop n xs)
    where
        n = read x :: Int
        xs' = map (map read . words) $ take n xs
        solve' :: [Int] -> [[Int]] -> Bool
        solve' set' (y:ys) = if y' `elem` set' then False else solve' (y':set') ys
            where
                y' = head y
        solve' _ [] =  True
solve _ [] = []

main :: IO ()
main = interact
    $ unlines
    . map (\b -> if b then "YES" else "NO")
    . solve []
    . tail
    . lines
