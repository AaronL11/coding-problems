import Text.Printf (printf)

-- This function should return a list [area, volume].
solve :: Int -> Int -> [Int] -> [Int] -> [Double]
solve l r as bs = [ans, ans2]
    where
        lft = fromIntegral l
        rgt = fromIntegral r
        dx = 0.001
        interval = [lft, lft+dx..rgt]
        f :: Double -> Double
        f x = sum
            $ zipWith (\a b -> if b >=0 then (fromIntegral a)*x^b else (fromIntegral a)/x^(-b)) as bs
        ans = sum $ map (\x -> (f x)*dx) interval
        ans2 = sum $ map (\x -> dx*pi*(f x)^2) interval

--Input/Output.
main :: IO ()
main = getContents
    >>= mapM_ (printf "%.1f\n")
    . (\[a, b, [l, r]] -> solve l r a b)
    . map (map read. words)
    . lines