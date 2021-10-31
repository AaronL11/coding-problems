import Data.List (sort)
main :: IO ()
main = interact $ unwords . map show . sort . map (read::String->Int) . words


