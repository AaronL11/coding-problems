
main :: IO()
main = interact
        $ unwords
        . (
            \([a,b,c,d,e,f,g,h],norm)
            -> map (show . (/ norm))
            [
                a*e-b*f-c*g-d*h,
                b*e+a*f-d*g+c*h,
                c*e+d*f+a*g-b*h,
                d*e-c*f+b*g+a*h
            ]
        ) . (
            \[a,b,c,d,e,f,g,h]
            -> (
                [a,b,c,d,e,-f,-g,-h],
                e**2 + f**2 + g**2 + h**2
           )
        ) . map (read::String->Double)
        . words



