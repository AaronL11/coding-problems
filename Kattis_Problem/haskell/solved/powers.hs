{-# LANGUAGE LambdaCase, FlexibleContexts, NumericUnderscores, TupleSections #-}

module Main where

import qualified Data.List as L
import qualified Data.Set as S
import qualified Data.Map as M
import qualified Data.ByteString.Lazy.Char8 as C
import qualified Data.Array as A

import Text.Read (readMaybe)

import Data.List.Split (chunksOf)

import Data.Maybe (isNothing, isJust, fromJust)
import Data.Char (isDigit, isSpace)

import Control.Applicative
import Control.Monad (replicateM)
import Control.Monad.State

import Control.Arrow

import Data.ByteString.Lazy (ByteString)
import Data.Word (Word8)

-- Scanner code

type Scanner = State [C.ByteString]

scan :: Scanner a -> C.ByteString -> a
scan s = evalState s . C.words

str :: Scanner C.ByteString
str = get >>= \case { [] -> return C.empty; s:ss -> put ss >> return s }

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
tillEnd s = get >>= \case { [] -> return []; _ -> (:) <$> s <*> tillEnd s }

two, three, four :: Scanner a -> Scanner [a]
[two, three, four] = replicateM <$> [2..4]

-- Helper types

m :: Integer
m = 10^9+7

newtype Mod = Mod { unMod :: Integer }
    deriving (Eq,Ord)
instance Show Mod where show = show . unMod

instance Num Mod where
    fromInteger n = Mod (n `mod` m)
    (Mod a) + (Mod b) = Mod ((a + b) `mod` m)
    (Mod a) - (Mod b) = Mod ((a - b) `mod` m)
    (Mod a) * (Mod b) = Mod ((a * b) `mod` m)
    abs           = undefined
    signum        = undefined

-- Helper functions

enumerate :: [a] -> [(Int,a)]
enumerate = L.zip [0..]

readInt :: C.ByteString -> Int
readInt = fst . fromJust . C.readInt

(?) = filter

-- Actual code

-- Main

main :: IO()
main = C.interact
        $ C.pack
        . show
        . (\[a,b] -> if even a then (div a 2)^b `mod` a else 0)
        . scan (tillEnd integer)
