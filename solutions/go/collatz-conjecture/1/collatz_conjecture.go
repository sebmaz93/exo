package collatzconjecture

import "errors"

func CollatzConjecture(n int) (int, error) {
	if n <= 0 {
		return 0, errors.New("only positive numbers expected.")
	}

	numbers := []int{}
	currentNum := n

	for currentNum > 1 {
		if currentNum%2 == 0 {
			currentNum /= 2
			numbers = append(numbers, currentNum)
		} else {
			currentNum = (currentNum * 3) + 1
			numbers = append(numbers, currentNum)
		}
	}

	return len(numbers), nil
}
