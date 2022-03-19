package hcl

import (
	"github.com/hashicorp/hcl/v2/hclwrite"
	"github.com/zclconf/go-cty/cty"
	"reflect"
)

func getMatchingBlocks(b *hclwrite.Body, typeName string, labels []string) []*hclwrite.Block {
	var matched []*hclwrite.Block
	for _, block := range b.Blocks() {
		if typeName == block.Type() {
			labelNames := block.Labels()
			if len(labels) == 0 && len(labelNames) == 0 {
				matched = append(matched, block)
				continue
			}
			if reflect.DeepEqual(labels, labelNames) {
				matched = append(matched, block)
			}
		}
	}

	return matched
}

func SetStringValue(f *hclwrite.File, blockType string, blockLabels []string, attributeName string, value string) {
	for _, m := range getMatchingBlocks(f.Body(), blockType, blockLabels) {
		m.Body().SetAttributeValue(attributeName, cty.StringVal(value))
	}
}
