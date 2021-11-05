main :: IO ()
main = interact
	$ ("Dr. Chaz " ++)
	. (\(d,s) -> if d>0
			then
			    "will have " ++ show d ++ " " ++ s ++ " of chicken left over!"
			else
			    "needs " ++ show (abs d) ++ " more " ++ s ++ " of chicken!"
		 )
	. (\d -> (d,if abs d==1 then "piece" else "pieces"))
	. (\[n,m] -> m-n)
	. map read
	. words

