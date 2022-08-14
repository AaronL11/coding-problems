main :: IO()
main = interact
        $ unlines
        . solve
        . map read
        . lines
        where solve (n:ns) = ns >>= (map show . take n . iterate (id))

