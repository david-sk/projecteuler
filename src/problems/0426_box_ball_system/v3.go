//
// Box-ball system, v3
// https://projecteuler.net/problem=426
//
// go run src/problems/0426_box_ball_system/v3.go
//

package main

import "fmt"

const sequenceChunk = 1000

type Boxball struct {
	Size          uint64
	Sequence      []uint64
	endingMessage chan string
}

func (b *Boxball) nextSequence(turnNumber uint64, nextMaxIndexMessages chan uint64) {
	var numBall uint64 = b.Sequence[0]
	var surplus uint64 = 0

	var start uint64 = 1

	childNextMaxIndexMessages := make(chan uint64)

	for {
		select {
		case nextMaxIndex := <-nextMaxIndexMessages:
			if nextMaxIndex == 0 {
				nextMaxIndex = b.Size
			}

			for i := uint64(start); i < nextMaxIndex; i++ {
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

				if i != b.Size-1 {
					if i == sequenceChunk {
						go b.nextSequence(turnNumber+1, childNextMaxIndexMessages)
						childNextMaxIndexMessages <- i
					} else if i%sequenceChunk == 0 {
						childNextMaxIndexMessages <- i
					}
				}
			}
			start = nextMaxIndex

			if nextMaxIndex == b.Size {
				if turnNumber == b.Size {
					b.endingMessage <- "THE_END"
				} else {
					b.Sequence[b.Size-1] = numBall + surplus
					childNextMaxIndexMessages <- 0
				}
				return
			}
		}
	}
}

func main() {
	var limit uint64 = 10000
	var s uint64 = 290797

	sequence := []uint64{}
	for i := uint64(0); i <= limit; i++ {
		sequence = append(sequence, s%64+1)
		s = (s * s) % 50515093
	}

	boxball := &Boxball{
		Size:          uint64(len(sequence)),
		Sequence:      sequence,
		endingMessage: make(chan string),
	}

	nextMaxIndexMessages := make(chan uint64)

	go boxball.nextSequence(0, nextMaxIndexMessages)
	nextMaxIndexMessages <- 0

	msg := <-boxball.endingMessage
	if msg == "THE_END" {
		if limit <= 10000 {
			fmt.Println("---------------------")
			for i := uint64(0); i < boxball.Size; i += 2 {
				fmt.Print(boxball.Sequence[i])
				fmt.Print(" ")
			}
			fmt.Println("\n---------------------")
		} else {
			var squareSum uint64 = 0
			for i := uint64(0); i < boxball.Size; i += 2 {
				squareSum += boxball.Sequence[i]
			}
			fmt.Println(squareSum)
		}
	}
}
