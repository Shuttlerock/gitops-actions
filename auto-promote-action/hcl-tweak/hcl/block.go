package hcl

import (
	"github.com/hashicorp/hcl/v2/hclwrite"
	"reflect"
	"shuttlerock.com/hcl-tweak/types"
)

// GetMatchingBlocks retrieves every block with matching type, labels and attributes.
func GetMatchingBlocks(
	b *hclwrite.Body,
	typeName string,
	labels []string,
	attributes []types.KeyValuePair,
) ([]*hclwrite.Block, error) {
	var matched []*hclwrite.Block

	for _, block := range b.Blocks() {
		if typeName != block.Type() {
			continue
		}

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

	return matched, nil
}
