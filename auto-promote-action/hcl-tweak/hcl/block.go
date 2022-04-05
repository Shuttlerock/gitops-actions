package hcl

import (
	"github.com/hashicorp/hcl/v2/hclwrite"
	"github.com/zclconf/go-cty/cty"
	"reflect"
	"shuttlerock.com/hcl-tweak/types"
)

func GetMatchingBlocks(b *hclwrite.Body, typeName string, labels []string, attributes []types.KeyValuePair) ([]*hclwrite.Block, error) {
	var matched []*hclwrite.Block
	for _, block := range b.Blocks() {
		if typeName == block.Type() {
			labelNames := block.Labels()

			if len(labels) > 0 && !reflect.DeepEqual(labels, labelNames) {
				continue
			}

			attributesMatch := true

			for _, attributePair := range attributes {
				attribute := block.Body().GetAttribute(attributePair.Key)

				if attribute == nil {
					attributesMatch = false
					break
				}

				value := GetAttributeValue(attribute)

				if value != attributePair.Value {
					attributesMatch = false
					break
				}
			}

			if !attributesMatch {
				continue
			}

			matched = append(matched, block)
		}
	}

	return matched, nil
}

func SetStringValue(f *hclwrite.File, blockType string, blockLabels []string, blockAttributes []types.KeyValuePair, attributeName string, value string) error {
	blocks, err := GetMatchingBlocks(f.Body(), blockType, blockLabels, blockAttributes)

	if err != nil {
		return err
	}

	for _, block := range blocks {
		block.Body().SetAttributeValue(attributeName, cty.StringVal(value))
	}

	return nil
}
