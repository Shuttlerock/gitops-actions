package hcl

import (
	"github.com/hashicorp/hcl/v2/hclwrite"
	"github.com/zclconf/go-cty/cty"
	"strings"
)

// GetAttributeValue returns a string representation of the value of the attribute.
func GetAttributeValue(attribute *hclwrite.Attribute) string {
	tokens := attribute.Expr().BuildTokens(nil)

	return strings.Trim(string(tokens.Bytes()), "\" ")
}

// SetAttributeValue updates the contents of a named attribute within the given block.
func SetAttributeValue(block *hclwrite.Block, name string, value string) {
	block.Body().SetAttributeValue(name, cty.StringVal(value))
}
