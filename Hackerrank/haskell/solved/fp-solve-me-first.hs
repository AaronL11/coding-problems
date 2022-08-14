main :: IO()
main = interact $ show . sum . map (read::String->Integer) . lines
