solve [] = []
solve (a:b:rest) = abs(a-b) : solve rest

main :: IO ()
main = interact $ unlines . map show . solve . map read . words 
