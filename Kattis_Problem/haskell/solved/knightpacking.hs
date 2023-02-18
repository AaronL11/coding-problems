main = interact
    $ (\x -> if odd x then "first" else "second")
    . read
