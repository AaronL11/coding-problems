solve :: Int -> Int
solve n
  | n==1 || n==0 = 0
  | n==2         = 1
  | otherwise    = (2^n) - (n+1)

main :: IO()
main = interact
        $ unwords
        . map (show . solve . read) 
        . words

