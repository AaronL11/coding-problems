fn :: Int -> [Int]
fn n = takeWhile (<n) $ iterate (+1) 0

main = do
n <- readLn :: IO Int
print (fn(n))
