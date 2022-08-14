{-# LANGUAGE LambdaCase, FlexibleContexts #-}

import Data.List as L
import Data.Set as S
import Data.Map as M
import Data.Maybe (isNothing, isJust)

import Control.Monad (replicateM)
import Control.Monad.State

-- Scanner code

type Scanner = State [String]

scan :: Scanner a -> String -> a
scan s = evalState s . words

str :: Scanner String
str = get >>= \case {s:ss -> put ss >> return s}

int :: Scanner Int
int = read <$> str

integer :: Scanner Integer
integer = read <$> str

float :: Scanner Float
float = read <$> str

double :: Scanner Double
double = read <$> str

grab :: Scanner a -> Scanner [a]
grab s = int >>= flip replicateM s

tillEnd :: Scanner a -> Scanner [a]
tillEnd s = get >>= \case { [] -> return []; _ -> (:) <$> s <*> tillEnd s}

two, three, four :: Scanner a -> Scanner [a]
[two, three, four] = L.map replicateM [2..4]

-- Helper functions

enumerate :: [a] -> [(Int,a)]
enumerate = zip [0..]

-- Actual code

type Election = [Int]

elections :: Scanner [Election]
elections = grab $ grab int

foldIt ::(Maybe Int,Int,Int) -> (Int,Int) -> (Maybe Int,Int,Int)
foldIt (i,m,s) (j,x)
  | x > m       = (Just j,x,s+x)
  | x == m      = (Nothing, x, s+x)
  | isNothing i = (Nothing, m, s+x)
  | otherwise   = (i, m, s+x)

analyse :: Election -> (Maybe Int, Int, Int)
analyse = L.foldl foldIt (Just 0, 0, 0) . enumerate

solve :: (Maybe Int, Int, Int) -> String
solve (i,m,s) = case i of 
                  Just i  -> if q > 0.5
                                then "majority winner " ++ show (i+1)
                                else "minority winner " ++ show (i+1)
                                    where
                                        q = fromIntegral m / fromIntegral s
                  Nothing -> "no winner"

main :: IO ()
main = interact
    $ unlines
    . fmap (solve . analyse)
    <$> scan (grab $ grab int)

