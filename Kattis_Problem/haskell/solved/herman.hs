main :: IO ()
main = interact $ unlines . map show . (\r -> [pi*r*r, r*r*2]) . read

