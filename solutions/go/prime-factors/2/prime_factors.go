package prime

func Factors(n int64) []int64 {
	fac := []int64{}
	num := n

	if n == 1 {
		return []int64{}
	}

	divisor := int64(2)

	for num > 1 {
		if num%divisor == 0 {
			fac = append(fac, divisor)
			num = num / divisor
		} else {
			divisor++
		}
	}
	return fac
}
