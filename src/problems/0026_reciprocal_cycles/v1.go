//
// Reciprocal cycles, v1
// https://projecteuler.net/problem=26
//
// A unit fraction contains 1 in the numerator. The decimal representation of the unit fractions
// with denominators 2 to 10 are given:
//     1/2  =  0.5
//     1/3  =  0.(3)
//     1/4  =  0.25
//     1/5  =  0.2
//     1/6  =  0.1(6)
//     1/7  =  0.(142857)
//     1/8  =  0.125
//     1/9  =  0.(1)
//     1/10 =  0.1
// Where 0.1(6) means 0.166666..., and has a 1-digit recurring cycle. It can be seen that 1/7 has
// a 6-digit recurring cycle.
// Find the value of d < 1000 for which 1/d contains the longest recurring cycle in its decimal
// fraction part.
//
// go run src/problems/0026_reciprocal_cycles/v1.go
//

package main

import (
	"math/big"
	"strconv"
	"strings"
)

func main() {
	// `prec` as in precision, but notice that it does _not_ correspond to the number of decimals.
	// NOTE: overall, the precision is arbitrary here, it was increased until the correct answer
	// was found, assuming the maximal about of digits a cycle can have is not known. This is
	// obviously a terrible way of handling this problem, a much simpler solution should exist!
	var prec uint = 8592

	maxCycleLength := 0
	solution := -1

	for d := 1; d < 1000; d++ {
		numerator, _ := new(big.Float).SetPrec(prec).SetString("1")
		denominator, _ := new(big.Float).SetPrec(prec).SetString(strconv.FormatInt(int64(d), 10))
		quotient := numerator.Quo(numerator, denominator)

		n := quotient.Text('f', -1) // quotient as string, named `n` to keep it short
		if len(n) < 4 {
			continue // skip "0.X" quotients as they can't have cycles
		}
		n = n[2 : len(n)-1] // removes "0." and the last digit as it's rounded, e.g., 0.1235 -> 123
		length := len(n)

		for i := 0; i < length; i++ {
			for j := i + 1; j < length; j++ {
				// skip all same digits cycles
				ok := false
				for k := i + 1; k < j; k++ {
					if n[k] != n[k-1] {
						ok = true
						break
					}
				}
				if !ok {
					continue
				}

				// try to find the same substring next, e.g., "142857" at index 6 in "142857142857"
				substring := n[i:j]
				if strings.Index(n[j:], substring) == 0 {
					if len(substring) > maxCycleLength {
						maxCycleLength = len(substring)
						solution = d
					}
					i = length // break outer loop
					break
				}
			}
		}
	}

	println("Result:", solution, " ( for a max cycle length of", maxCycleLength, ")")
}
