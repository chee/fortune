package main

import (
	"fmt"
	"io/ioutil"
	"math/rand"
	"strings"
)

func main() {
	text, _ := ioutil.ReadFile("fortunes")
	fortunes := strings.Split(string(text), "\n%\n")
	fmt.Println(fortunes[rand.Intn(len(fortunes))])
}
