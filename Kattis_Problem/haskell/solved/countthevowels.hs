solve :: Char -> Int
solve n
  | n `elem` "aeiouAEIOU" = 1
  | otherwise           = 0

main :: IO ()
main = interact
    $ unlines
    . map (show . sum . map solve)
    . lines

