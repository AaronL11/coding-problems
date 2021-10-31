main :: IO()
main = interact $ (\x -> if odd x then "Alice" else "Bob") . head . map read . words

