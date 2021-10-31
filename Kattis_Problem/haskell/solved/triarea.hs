main :: IO ()
main = interact $ show . (/2) . sum . take 2 . map read . words

