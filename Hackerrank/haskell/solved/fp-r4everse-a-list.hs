rev :: [a] -> [a]
rev l = case l of
        []     -> []
        [x]    -> [x]
        [x,y]  -> [y,x]
        (x:xs) -> rev xs ++ [x]
