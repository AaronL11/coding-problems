fib n
    | n == 1 || n == 2 = n-1
    | otherwise        = fib (n-1) + fib (n-2)

main = interact $ show . fib . read