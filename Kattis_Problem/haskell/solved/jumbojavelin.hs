main :: IO()
main = interact $ show . (\(n:xs) -> sum xs+1-n) . map read . words

