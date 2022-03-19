package main

import (
	"flag"
	"fmt"
	"os"
	"shuttlerock.com/hcl-tweak/hcl"
	"strings"
)

type arrayFlag []string

func (i *arrayFlag) String() string {
	return strings.Join(*i, " ")
}

func (i *arrayFlag) Set(value string) error {
	*i = append(*i, value)
	return nil
}

func main() {
	var labels arrayFlag

	filename := flag.String("filename", "", "Path to HCL file to modify.")
	block := flag.String("block", "", "Name of block to modify.")
	flag.Var(&labels, "labels", "List of labels for the block to modify.")
	attribute := flag.String("attribute", "", "Name of the attribute to modify.")
	value := flag.String("value", "", "New value for the attribute.")
	flag.Parse()

	if *filename == "" || *block == "" || *attribute == "" || *value == "" {
		flag.Usage()
		os.Exit(1)
	}

	file, err := hcl.OpenFile(*filename)

	if err != nil {
		fmt.Fprintf(os.Stderr, "Failed to open file: %s", err)
	}

	hcl.SetStringValue(file, *block, labels, *attribute, *value)

	err = hcl.SaveFile(file, *filename)

	if err != nil {
		fmt.Fprintf(os.Stderr, "Failed to save file: %s", err)
	}
}
