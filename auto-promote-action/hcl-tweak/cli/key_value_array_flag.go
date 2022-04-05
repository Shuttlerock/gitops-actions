package cli

import (
	"errors"
	"fmt"
	"shuttlerock.com/hcl-tweak/types"
	"strings"
)

// KeyValueArrayFlag allows an array of key value pairs of the form `x=y` to be specified when running the application.
type KeyValueArrayFlag []types.KeyValuePair

func (i *KeyValueArrayFlag) String() string {
	result := ""

	for _, pair := range *i {
		result += fmt.Sprintf("%s=%s", pair.Key, pair.Value)
	}

	return result
}

func (i *KeyValueArrayFlag) Set(value string) error {
	parts := strings.Split(value, "=")

	if len(parts) != 2 {
		return errors.New("invalid key value pair specified")
	}

	*i = append(*i, types.KeyValuePair{Key: parts[0], Value: parts[1]})

	return nil
}
