main :: IO ()
main = interact
	$ (\[n,p,q] ->  if even ((p+q) `div` n) then "paul" else "opponent")
	. map read
	. words

