main :: IO ()
main = interact
    $ show
	. (*4)
	. sqrt
	. read

