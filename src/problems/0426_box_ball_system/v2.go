//
// Box-ball system, v2
// https://projecteuler.net/problem=426
//
// go run src/problems/0426_box_ball_system/v2.go
//

package main

import "fmt"

type Boxball struct {
	Size     uint64
	Sequence []uint64
}

func (b *Boxball) nextSequence() {
	var numBall uint64 = b.Sequence[0]
	var surplus uint64 = 0

	for i := uint64(1); i < b.Size; i++ {
		if i%2 == 1 { // odd
			if b.Sequence[i] <= numBall {
				numBall -= b.Sequence[i]
				b.Sequence[i-1] = b.Sequence[i] + surplus
				surplus = 0
			} else {
				surplus = b.Sequence[i] - numBall
				b.Sequence[i-1] = numBall
				numBall = 0
			}
		} else { // even
			numBall += b.Sequence[i]
			b.Sequence[i-1] = b.Sequence[i] + surplus
			surplus = 0
		}
	}
	b.Sequence[b.Size-1] = numBall + surplus
}

func main() {
	var limit uint64 = 10000
	var s uint64 = 290797

	sequence := []uint64{}
	for i := uint64(0); i <= limit; i++ {
		sequence = append(sequence, s%64+1)
		s = (s * s) % 50515093
	}

	boxball := Boxball{Size: uint64(len(sequence)), Sequence: sequence}

	for i := uint64(0); i < limit; i++ {
		boxball.nextSequence()
	}
	if limit <= 10000 {
		fmt.Println(boxball.Sequence)
	}
}
