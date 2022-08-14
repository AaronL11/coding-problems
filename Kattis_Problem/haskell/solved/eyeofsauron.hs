
collapse :: (Int,Bool) -> Char -> (Int,Bool)
collapse (a,b) x = case x of 
      '|' -> if b then (a-1,b) else (a+1,b)
      '(' -> (a,b)
      ')' -> (a,True)

solve :: (Int,Bool) -> String
solve (n,_)
  | n == 0    = "correct"
  | otherwise = "fix"

main :: IO ()
main = interact
    $ unlines
    . map (solve . foldl collapse (0,False))
    . lines



