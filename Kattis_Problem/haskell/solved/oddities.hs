main :: IO ()
main = interact $ unlines
		.map ((\i -> show i ++ if even i then " is even" else " is odd") . read)
		. tail
		. lines

