package cli

import "strings"

// ArrayFlag allows an array of values to be specified when running the application.
type ArrayFlag []string

func (i *ArrayFlag) String() string {
	return strings.Join(*i, " ")
}

func (i *ArrayFlag) Set(value string) error {
	*i = append(*i, value)
	return nil
}
