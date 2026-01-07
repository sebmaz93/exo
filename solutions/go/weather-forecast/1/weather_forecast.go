// Package weather provides tools to get
// weather forecast.
package weather

var (
	// CurrentCondition is where we store the city forecast condition.
	CurrentCondition string
	// CurrentLocation is to store the location of the user.
	CurrentLocation string
)

// Forecast returns a string showing current location and current weather condition with formatted text.
func Forecast(city, condition string) string {
	CurrentLocation, CurrentCondition = city, condition
	return CurrentLocation + " - current weather condition: " + CurrentCondition
}
