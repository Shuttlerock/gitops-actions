package hcl

import (
	"github.com/hashicorp/hcl/v2/hclwrite"
	"strings"
)

func GetAttributeValue(attribute *hclwrite.Attribute) string {
	tokens := attribute.Expr().BuildTokens(nil)

	return strings.Trim("\"", string(tokens.Bytes()))
}
