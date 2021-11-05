counter :: Char -> Char
counter a
  | a == 'R' = 'S'
  | a == 'B' = 'K'
  | a == 'L' = 'H'
  | otherwise = undefined

solve :: String -> String
solve "" = ""
solve [a] = [counter a]
solve [a,b] = [counter a, counter b]
solve [a,b,c]
  | a == 'R' && ((b == 'L' && c == 'B') || (b == 'B' && c == 'L')) = ['C']
  | a == 'B' && ((b == 'L' && c == 'R') || (b == 'R' && c == 'L')) = ['C']
  | a == 'L' && ((b == 'R' && c == 'B') || (b == 'B' && c == 'R')) = ['C']
  | otherwise = [counter a, counter b, counter c]
solve (a:b:c:rest)
  | a == 'R' && ((b == 'L' && c == 'B') || (b == 'B' && c == 'L')) = 'C':solve rest
  | a == 'B' && ((b == 'L' && c == 'R') || (b == 'R' && c == 'L')) = 'C':solve rest
  | a == 'L' && ((b == 'R' && c == 'B') || (b == 'B' && c == 'R')) = 'C':solve rest
  | otherwise = counter a : solve (b:c:rest)

main :: IO()
main = interact
	$ unwords
	. map solve
	. words

