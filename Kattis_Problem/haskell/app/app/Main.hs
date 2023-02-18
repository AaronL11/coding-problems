{-# LANGUAGE LambdaCase, FlexibleContexts, NumericUnderscores, TupleSections #-}

module Main where

import qualified Data.List as L
import qualified Data.Set as S
import qualified Data.Map as M
import qualified Data.ByteString.Lazy.Char8 as C
import qualified Data.Array as A

import Data.Maybe (catMaybes)

import Text.Read (readMaybe)

import Data.List.Split (chunksOf)

import Data.Maybe (isNothing, isJust, fromJust)
import Data.Char (isDigit, isSpace, chr)

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

primes :: [Integer]
primes = 2 : sieve primes [3..]
  where
    sieve (p:ps) xs =
        let (h,tj) = span (< p*p) xs
      in  h ++ sieve ps (filter ((/=0).(`mod`p)) tj)

main :: IO ()
main = C.interact $ solve . scan str
    where
        solve :: C.ByteString -> C.ByteString
        solve = C.pack
                . (\x -> if x=="Hey" then "hmm" else x)
                . rev
                . foldl1 min
                . catMaybes
                . map parse
                . L.permutations
                . C.unpack
        parse :: String -> Maybe Int
        parse ('X':'X':'X':cs)      = filt cs 30
        parse ('X':'X':cs)          = filt cs 20
        parse ('X':'L':cs)          = filt cs 40
        parse ('X':'C':cs)          = filt cs 90
        parse ('X':cs)              = filt cs 10
        parse ('L':'X':'X':'X':cs)  = filt cs 80
        parse ('L':'X':'X':cs)      = filt cs 70
        parse ('L':'X':cs)          = filt cs 60
        parse ('L':cs)              = filt cs 50
        parse ('C':_)               = Nothing
        parse s = case s of
                    []      -> Just 0
                    "I"     -> Just 1
                    "II"    -> Just 2
                    "III"   -> Just 3
                    "IV"    -> Just 4
                    "V"     -> Just 5
                    "VI"    -> Just 6
                    "VII"   -> Just 7
                    "VIII"  -> Just 8
                    "IX"    -> Just 9
                    "X"     -> Just 10
                    "XX"    -> Just 20
                    "XXX"   -> Just 30
                    "XL"    -> Just 40
                    "L"     -> Just 50
                    "LX"    -> Just 60
                    "LXX"   -> Just 70
                    "LXXX"  -> Just 80
                    "XC"    -> Just 90
                    _       -> Nothing
        filt :: String -> Int -> Maybe Int
        filt cs n = case cs of
                    ('C':_) -> Nothing
                    ('X':_) -> Nothing
                    ('L':_) -> Nothing
                    _       -> Just (+n) <*> parse cs
        rev :: Int -> String
        rev n = case n of
                  1  -> "I"
                  2  -> "II"
                  3  -> "III"
                  4  -> "IV"
                  5  -> "V"
                  6  -> "VI"
                  7  -> "VII"
                  8  -> "VIII"
                  9  -> "IX"
                  10 -> "X"
                  20 -> "XX"
                  30 -> "XXX"
                  40 -> "XL"
                  50 -> "L"
                  60 -> "LX"
                  70 -> "LXX"
                  80 -> "LXXX"
                  90 -> "XC"
                  _  -> rev ((div n 10) * 10) <> rev (mod n 10)

