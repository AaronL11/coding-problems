import Data.List

split :: Char -> String -> [String]
split s [] = []
split s (c:cs)
  | c == s = "":rest
  | otherwise = (c : head rest) : tail rest
	where
	rest = split s cs

solve :: String -> String -> String -> String
solve d b c
  | date >= 2010 || born >= 1991 = "eligible"
  | courses >= 41 = "ineligible"
  | otherwise = "coach petitions"
    where
	date = read d
	born = read b
	courses = read c



main :: IO()
main = interact
	$ unlines
	. map (
		unwords
		. (\[n,d,b,c] -> [n , solve d b c])
		. (\[n,d,b,c] ->[n
				, fst (break (=='/') d)
				, fst (break (=='/') b)
				, c])
       		. words
       	)
	. tail
	. lines


