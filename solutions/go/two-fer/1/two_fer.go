// Package twofer
//
// it provides a func ShareWith which gives a message based on the name or no name
// it can be used for
package twofer

import "fmt"

// ShareWith is function takes the name of the person to share with
// and returns you sentence with the person name, or if no name it returns generic message.
func ShareWith(name string) string {
	person := "you"
	if name != "" {
		person = name
	}
	return fmt.Sprintf("One for %s, one for me.", person)
}
