import Data.List


solve :: Int -> Int -> Int
solve x c
  | x < c     = 0
  | otherwise = q + solve (q+r) c
  where
      q = x `div` c
      r = x `mod` c

main :: IO ()
main = interact
    $ show . (\[e,f,c] -> solve (e+f) c)
    . map (read::String->Int) . words
