main :: IO()
main = interact
        $ unlines
        . map (show . snd)
        . filter (even . fst)
        . zip [1..]
        . map (read::String->Int)
        . lines
