module LogicGate where

-- Бесконечный поток простых чисел для верификации системы
primes :: [Integer]
primes = sieve [2..]
  where
    sieve (p:xs) = p : sieve [x | x <- xs, x `mod` p /= 0]

-- Проверка статуса суверена через математическое сито
checkStatus :: Integer -> Bool
checkStatus n = n `elem` take (fromIntegral n) primes
