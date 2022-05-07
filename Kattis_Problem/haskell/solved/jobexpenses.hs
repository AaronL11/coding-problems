main :: IO()
main = interact
    $ show
    . sum
    . map (*(-1))
    . filter (<0)
    . map read
    . drop 1
    .words

