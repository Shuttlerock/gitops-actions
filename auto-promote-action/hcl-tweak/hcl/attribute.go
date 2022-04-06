package hcl

import (
	"github.com/hashicorp/hcl/v2/hclwrite"
	"github.com/zclconf/go-cty/cty"
	"strings"
)

// GetAttributeValue returns a string representation of a named attribute within the given block.
func GetAttributeValue(block *hclwrite.Block, name string) *string {
	attribute := block.Body().GetAttribute(name)

	if attribute == nil {
		return nil
	}

	tokens := attribute.Expr().BuildTokens(nil)

	result := strings.Trim(string(tokens.Bytes()), "\" ")

	return &result
}

// SetAttributeValue updates the contents of a named attribute within the given block.
func SetAttributeValue(block *hclwrite.Block, name string, value string) {
	block.Body().SetAttributeValue(name, cty.StringVal(value))
}
