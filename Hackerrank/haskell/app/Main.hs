module Main where

main :: IO()
main = interact
        $ unlines
        . flip take (iterate (const "Hello World"))
        $ read
