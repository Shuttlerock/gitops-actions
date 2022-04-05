package main

import (
	"flag"
	"fmt"
	"os"
	"shuttlerock.com/hcl-tweak/cli"
	"shuttlerock.com/hcl-tweak/hcl"
)

func main() {
	var labels cli.ArrayFlag
	var attributes cli.KeyValueArrayFlag

	filename := flag.String("filename", "", "Path to HCL file to modify.")
	block := flag.String("block", "", "Name of block to modify.")
	flag.Var(&labels, "label", "List of labels that must be present in a block.")
	flag.Var(&attributes, "attribute", "List of attributes that must be present in a block.")
	targetAttribute := flag.String("target-attribute", "", "Name of the target attribute to modify.")
	targetValue := flag.String("target-value", "", "New value for the attribute.")
	flag.Parse()

	if *filename == "" || *block == "" || *targetAttribute == "" || *targetValue == "" {
		flag.Usage()
		os.Exit(1)
	}

	file, err := hcl.OpenFile(*filename)

	if err != nil {
		fmt.Fprintf(os.Stderr, "Failed to open file: %s", err)
		os.Exit(1)
	}

	err = hcl.SetStringValue(file, *block, labels, attributes, *targetAttribute, *targetValue)

	if err != nil {
		fmt.Fprintf(os.Stderr, "Failed to update attributes: %s", err)
		os.Exit(1)
	}

	err = hcl.SaveFile(file, *filename)

	if err != nil {
		fmt.Fprintf(os.Stderr, "Failed to save file: %s", err)
		os.Exit(1)
	}
}
