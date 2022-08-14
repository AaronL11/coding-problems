{-# LANGUAGE LambdaCase, FlexibleContexts #-}

import qualified Data.List as L
import qualified Data.Set as S
import qualified Data.Map as M
import qualified Data.ByteString.Lazy.Char8 as C

import Data.List.Split (chunksOf)

import Data.Maybe (isNothing, isJust, fromJust)

import Control.Monad (replicateM)
import Control.Monad.State

-- Scanner code

type Scanner = State [C.ByteString]

scan :: Scanner a -> C.ByteString -> a
scan s = evalState s . C.words

str :: Scanner C.ByteString
str = get >>= \case {s:ss -> put ss >> return s}

int :: Scanner Int
int = read . C.unpack <$> str

integer :: Scanner Integer
integer = read . C.unpack <$> str

float :: Scanner Float
float = read . C.unpack <$> str

double :: Scanner Double
double = read . C.unpack <$> str

grab :: Scanner a -> Scanner [a]
grab s = int >>= flip replicateM s

tillEnd :: Scanner a -> Scanner [a]
tillEnd s = get >>= \case { [] -> return []; _ -> (:) <$> s <*> tillEnd s}

two, three, four :: Scanner a -> Scanner [a]
[two, three, four] = L.map replicateM [2..4]

-- Helper functions

enumerate :: [a] -> [(Int,a)]
enumerate = L.zip [0..]

readInt :: C.ByteString -> Int
readInt = fst . fromJust . C.readInt

-- Actual code

solve :: [[Int]] -> C.ByteString
solve [gz,mgz] = case compare (maximum gz) (maximum mgz) of
                   LT -> C.pack "MechaGodzilla"
                   _ -> C.pack "Godzilla"

main :: IO ()
main = C.interact
    $ C.unlines
    . map (solve . map (map readInt . C.words) . drop 2)
    . chunksOf 4
    . drop 1
    . C.lines

