module Main where

solve :: Double -> Double
solve x = foldl (\acc i -> 1.0 + x*acc/i) 1.0
    . reverse
    $ map fromIntegral [1..9]

main = interact
    $ unlines
    . map (show . solve . read)
    . tail
    . lines