main :: IO()
main = interact
    $ unwords
    . map show
    . (\n -> [1/n * 100, 1/(100-n) * 100])
    . (read::String->Double)
