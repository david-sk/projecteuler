//
// Powerful digit sum, v1
// https://projecteuler.net/problem=56
//
// A googol (10^100) is a massive number: one followed by one-hundred zeros; 100^100 is almost
// unimaginably large: one followed by two-hundred zeros. Despite their size, the sum of the
// digits in each number is only 1.
// Considering natural numbers of the form, a^b, where a, b < 100, what is the maximum digital sum?
//
// go run src/problems/0056_powerful_digit_sum/v1.go
//

package main

import "math/big"

func getDigitsSum(n big.Int) int64 {
	var digitsSum int64
	zero, ten := big.NewInt(0), big.NewInt(10)
	for n.Cmp(zero) == 1 {
		var mod big.Int
		mod.Mod(&n, ten)
		digitsSum += mod.Int64()
		n.Div(&n, ten)
	}
	return digitsSum
}

func main() {
	var max int64
	var n big.Int
	for i := 1; i < 100; i++ {
		for j := 1; j < 100; j++ {
			n.Exp(big.NewInt(int64(i)), big.NewInt(int64(j)), nil)
			if digitsSum := getDigitsSum(n); max < digitsSum {
				max = digitsSum
			}
		}
	}
	println("Result:", max)
}
