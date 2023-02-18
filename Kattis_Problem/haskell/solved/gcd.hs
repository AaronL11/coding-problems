main :: IO()
main = interact
        $ show
        . (\[a,b] -> gcd a b)
        . map read
        . words
