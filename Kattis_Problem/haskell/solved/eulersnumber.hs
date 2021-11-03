solve :: Double -> Double
solve 0 = 1
solve n = 1.0/product [1..n] + solve (n-1)

main :: IO ()
main = interact
        $ show
	. solve
	. read

