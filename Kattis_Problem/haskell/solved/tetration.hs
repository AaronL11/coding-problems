main :: IO ()
main = interact $ show . (\n -> n ** recip n) . (read::String->Double)
