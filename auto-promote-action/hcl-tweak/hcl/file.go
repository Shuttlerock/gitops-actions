package hcl

import (
	"errors"
	"github.com/hashicorp/hcl/v2"
	"github.com/hashicorp/hcl/v2/hclwrite"
	"os"
)

func OpenFile(filename string) (*hclwrite.File, error) {
	bytes, err := os.ReadFile(filename)

	if err != nil {
		return nil, err
	}

	file, diag := hclwrite.ParseConfig(bytes, filename, hcl.Pos{Line: 1, Column: 1})

	if diag.HasErrors() {
		err = errors.New(diag.Error())
	}

	return file, err
}

func SaveFile(file *hclwrite.File, filename string) error {
	return os.WriteFile(filename, file.Bytes(), 0666)
}
