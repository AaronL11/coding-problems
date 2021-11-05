solve :: [String] -> String
solve [a,b,c]
  | [a,b,c]==["South","West","North"]				= "Yes"
  | [a,b,c]==["South","West","East"]				= "Yes"
  | [a,b,c]==["South","North","East"]				= "Yes"
  | [a,b,c]==["East","South","West"]				= "Yes"
  | [a,b,c]==["East","South","North"]				= "Yes"
  | [a,b,c]==["East","West","North"]				= "Yes"
  | [a,b,c]==["North","East","South"]				= "Yes"
  | [a,b,c]==["North","East","West"]				= "Yes"
  | [a,b,c]==["North","South","West"]				= "Yes"
  | [a,b,c]==["West","North","East"]				= "Yes"
  | [a,b,c]==["West","North","South"]				= "Yes"
  | [a,b,c]==["West","East","South"]				= "Yes"
  | otherwise 							= "No"

main :: IO ()
main = interact
	$ solve
	. words

