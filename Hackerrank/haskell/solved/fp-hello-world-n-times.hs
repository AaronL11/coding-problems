main :: IO()
main = interact
        $ unlines
        . flip take (iterate (id) "Hello World")
        . read
