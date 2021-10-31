fizz Int = String
fizz a = 


solve [] = []
solve (f:b:n) = map buzz . map fizz . [0..n]


main :: IO ()
main = interact $ unlines . map show . solve . map read . words


