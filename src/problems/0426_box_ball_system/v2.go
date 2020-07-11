package main

import "fmt"

// [46, 24, 9, 1, 21, 51, 54, 6, 3, 7, 31]
// [24, 9, 1, 21, 51, 54, 6, 3, 7, 31, 75]
// [9, 1, 16, 56, 51, 9, 3, 7, 10, 96, 75]
// [1, 16, 24, 83, 9, 3, 7, 10, 48, 123, 75]
// [1, 39, 24, 68, 3, 7, 10, 48, 51, 147, 75]
// [1, 62, 24, 47, 3, 14, 10, 89, 51, 171, 75]
// [1, 85, 24, 26, 3, 21, 10, 130, 51, 195, 75]
// [1, 108, 24, 5, 3, 28, 10, 171, 51, 219, 75]
// [1, 131, 5, 3, 22, 16, 10, 212, 51, 243, 75]
// [1, 135, 3, 22, 16, 10, 18, 245, 51, 267, 75]
// [1, 137, 3, 35, 10, 18, 24, 272, 51, 291, 75]

// => [1, 3, 10, 24, 51, 75]

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
	sequence := []uint64{46, 24, 9, 1, 21, 51, 54, 6, 3, 7, 31}
	boxball := Boxball{Size: uint64(len(sequence)), Sequence: sequence}
	fmt.Println(boxball.Sequence)

	for i := 0; i < 10; i++ {
		boxball.nextSequence()
		fmt.Println(boxball.Sequence)
	}
}
