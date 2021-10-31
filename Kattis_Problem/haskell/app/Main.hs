solve :: Int -> [String]
solve i = [show $ sum (take i [1..]),show $ sum (take i [1,3..]),show $ sum (take i [2,4..])]

main :: IO ()
main = interact
	$ unlines
	. map (unwords
		. (\l -> show (head l) : solve (last l)) 
		. map (read::String->Int)
		. words)
	. tail
	. lines
