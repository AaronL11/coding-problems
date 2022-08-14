solve :: [Int] -> Int
solve [a,b] = if a > b then 1 else 0

main :: IO ()
main = interact $ show . solve . map read . words

