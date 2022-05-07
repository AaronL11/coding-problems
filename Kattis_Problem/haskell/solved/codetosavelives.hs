intlist :: Int -> [Int]
intlist n
    |n == 0     = []
    |otherwise  = (n `mod` 10):(intlist (n `div` 10))

combine :: [Int] -> [Int]
combine [a,b]       = [a+b]
combine (a:b:chars) = (a+b):(combine chars)

main :: IO()
main = interact $
        unlines
        . map (reverse . unwords . map show . intlist)
        . combine
        . map (foldl (\n acc -> n*10 + acc) 0 . map read . words)
        . tail
        . lines


