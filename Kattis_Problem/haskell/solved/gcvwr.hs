
main :: IO ()
main = interact
    $ show
    . (\(g:t:_:ns) -> (g-t)*9 `div` 10 - sum ns)
    . map (read::String->Int)
    . words

