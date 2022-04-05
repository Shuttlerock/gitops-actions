package hcl

import (
	"errors"
	"github.com/hashicorp/hcl/v2"
	"github.com/hashicorp/hcl/v2/hclwrite"
	"os"
)

// ReadFile attempts to read and parse an HCL file into an AST for further processing.
func ReadFile(filename string) (*hclwrite.File, error) {
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

// WriteFile writes changes in the AST back to the filesystem.
func WriteFile(file *hclwrite.File, filename string) error {
	return os.WriteFile(filename, file.Bytes(), 0666)
}
