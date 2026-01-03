package raindrops

import "strconv"

func Convert(number int) string {
	text := ""
	if number%3 == 0 {
		text += "Pling"
	}
	if number%5 == 0 {
		text += "Plang"
	}
	if number%7 == 0 {
		text += "Plong"
	}
	if text == "" {
		return strconv.Itoa(number)
	}
	return text
}
