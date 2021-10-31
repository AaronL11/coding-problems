main :: IO()
main = interact $ last . take 2 . words

