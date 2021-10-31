import qualified Data.Set as Set

solve :: [String] -> [[String]]
solve [] = []
solve (n:rest) = take (read n :: Int) rest
		: solve (drop (read n :: Int) rest)

main :: IO()
main = interact 
	$ unlines
	. map (show .length . Set.toList . Set.fromList)
	. solve
	. tail
	. words

