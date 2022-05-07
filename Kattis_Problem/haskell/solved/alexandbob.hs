main :: IO()
main = interact
        $ (\[k,m,n] -> if k `mod` (m+n) >= m
                         then "Alex"
                         else "Bob")
        . map read
        . words

