main :: IO ()
main = interact $
    (\[m,a,b,c] -> if a+b+c<=2*m
                    then "possible"
                    else "impossible")
    . map read
    . words
